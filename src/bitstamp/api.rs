//! Use this module to interact with Bitstamp exchange.
//! Please see examples for more informations.

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header::ContentType;
use hyper::net::HttpsConnector;

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use error::*;
use helpers;
use bitstamp::utils;
use pair::Pair;

header! {
    #[doc(hidden)]
    (KeyHeader, "Key") => [String]
}

header! {
    #[doc(hidden)]
    (SignHeader, "Sign") => [String]
}

header! {
    #[doc(hidden)]
    (ContentHeader, "Content-Type") => [String]
}

#[derive(Debug)]
pub struct BitstampApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    customer_id: String,
    http_client: Client,
}


impl BitstampApi {
    /// Create a new BitstampApi by providing an API key & API secret
    pub fn new(params: &HashMap<&str, &str>) -> Result<BitstampApi> {
        let mut params = params.clone();
        helpers::strip_empties(&mut params);

        let empty_str: &str = "";

        let api_key = params.get("api_key").unwrap_or(&empty_str);
        let api_secret = params.get("api_secret").unwrap_or(&empty_str);
        let customer_id = params.get("customer_id").unwrap_or(&empty_str);

        //TODO: Handle correctly TLS errors with error_chain.
        let ssl = match NativeTlsClient::new() {
            Ok(res) => res,
            Err(_) => return Err(ErrorKind::TlsError.into()),
        };

        let connector = HttpsConnector::new(ssl);

        Ok(BitstampApi {
               last_request: 0,
               api_key: api_key.to_string(),
               api_secret: api_secret.to_string(),
               customer_id: customer_id.to_string(),
               http_client: Client::with_connector(connector),
           })
    }

    /// Create a new BitstampApi from a json configuration file. This file must follow this
    /// structure:
    ///
    /// ```ignore
    /// {
    ///     "account_kraken": {
    ///         "exchange"  : "kraken",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef"
    ///     },
    ///     "account_bitstamp": {
    ///         "exchange"   : "bitstamp",
    ///         "api_key"    : "1234567890ABCDEF1234567890ABCDEF",
    ///         "api_secret" : "1234567890ABCDEF1234567890ABCDEF",
    ///         "customer_id": "123456"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file("account_bitstamp", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> Result<BitstampApi> {
        let mut f = File::open(&path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let data: Value = serde_json::from_str(&buffer)?;
        let json_obj = data.as_object()
            .ok_or(ErrorKind::BadParse)?
            .get(config_name)
            .ok_or(ErrorKind::MissingField(config_name.to_string()))?;
        let api_key = json_obj
            .get("api_key")
            .ok_or(ErrorKind::MissingField("api_key".to_string()))?
            .as_str()
            .ok_or(ErrorKind::InvalidFieldFormat("api_key".to_string()))?;
        let api_secret = json_obj
            .get("api_secret")
            .ok_or(ErrorKind::MissingField("api_secret".to_string()))?
            .as_str()
            .ok_or(ErrorKind::InvalidFieldFormat("api_secret".to_string()))?;
        let customer_id = json_obj
            .get("customer_id")
            .ok_or(ErrorKind::MissingField("customer_id".to_string()))?
            .as_str()
            .ok_or(ErrorKind::InvalidFieldFormat("customer_id".to_string()))?;

        let mut params = HashMap::new();
        params.insert("api_key", api_key);
        params.insert("api_secret", api_secret);
        params.insert("customer_id", customer_id);

        Ok(BitstampApi::new(&params)?)
    }

    fn public_query(&mut self, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {

        let method: &str = params.get("method").ok_or("Missing \"method\" field.")?;
        let pair: &str = params.get("pair").ok_or("Missing \"pair\" field.")?;
        let url: String = utils::build_url(method, pair);

        utils::block_or_continue(self.last_request);
        let mut response = self.http_client.get(&url).send()?;
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        utils::deserialize_json(buffer)
    }

    ///
    ///
    /// #Examples
    ///
    /// ```ignore
    /// extern crate coinnect;
    /// use coinnect::bitstamp::BitstampApi;
    /// let mut api = BitstampApi::new("", "");
    /// let  result = api.private_query("balance", "btcusd");
    /// assert_eq!(true, true);
    /// ```
    fn private_query(&mut self, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {

        let method: &str = params.get("method").ok_or("Missing \"method\" field.")?;
        let pair: &str = params.get("pair").ok_or("Missing \"pair\" field.")?;
        let url: String = utils::build_url(method, pair);

        let nonce = utils::generate_nonce(None);
        let signature = utils::build_signature(nonce.clone(),
                                               self.customer_id.clone(),
                                               self.api_key.clone(),
                                               self.api_secret.clone())?;

        let copy_api_key = self.api_key.clone();
        let mut post_params: &mut HashMap<&str, &str> = &mut HashMap::new();
        post_params.insert("key", &copy_api_key);
        post_params.insert("signature", &signature);
        post_params.insert("nonce", &nonce);
        helpers::strip_empties(&mut post_params);
        let post_data = helpers::url_encode_hashmap(&post_params);
        let mut response = self.http_client
            .post(&url)
            .header(ContentType::form_url_encoded())
            .body(&post_data)
            .send()?;

        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        utils::deserialize_json(buffer)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {
    /// "BTC_LTC":{
    /// "last":"0.0251","lowestAsk":"0.02589999","highestBid":"0.0251",
    /// "percentChange":"0.02390438","baseVolume":"6.16485315","quoteVolume":"245.82513926"},
    /// "BTC_NXT":{
    /// "last":"0.00005730","lowestAsk":"0.00005710","highestBid":"0.00004903",
    /// "percentChange":"0.16701570","baseVolume":"0.45347489","quoteVolume":"9094"},
    /// ... }
    /// ```
    pub fn return_ticker(&mut self, pair: Pair) -> Result<Map<String, Value>> {
        let currency_pair = match pair {
            Pair::BTC_USD => "btcusd",
            _ => panic!("Unknown pair"),
        };

        let mut params = HashMap::new();
        params.insert("pair", currency_pair);
        params.insert("method", "ticker");
        self.public_query(&params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"asks":[[0.00007600,1164],[0.00007620,1300], ... ], "bids":[[0.00006901,200],
    /// [0.00006900,408], ... ], "timestamp": "1234567890"}
    /// ```
    pub fn return_order_book(&mut self, pair: Pair) -> Result<Map<String, Value>> {

        let currency_pair = match pair {
            Pair::BTC_USD => "btcusd",
            _ => panic!("Unknown pair"),
        };

        let mut params = HashMap::new();
        params.insert("method", "order_book");
        params.insert("pair", currency_pair);
        self.public_query(&params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// [{"date":"2014-02-10 04:23:23","type":"buy","rate":"0.00007600","amount":"140",
    /// "total":"0.01064"},
    /// {"date":"2014-02-10 01:19:37","type":"buy","rate":"0.00007600","amount":"655",
    /// "total":"0.04978"}, ... ]
    /// ```
    pub fn return_trade_history(&mut self, pair: Pair) -> Result<Map<String, Value>> {

        let currency_pair = match pair {
            Pair::BTC_USD => "btcusd",
            _ => panic!("Unknown pair"),
        };

        let mut params = HashMap::new();
        params.insert("pair", currency_pair);
        params.insert("method", "transactions");
        self.public_query(&params)
    }


    /// Returns all of your available balances.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"BTC":"0.59098578","LTC":"3.31117268", ... }
    /// ```
    pub fn return_balances(&mut self, pair: Pair) -> Result<Map<String, Value>> {

        let currency_pair = match pair {
            Pair::BTC_USD => "btcusd",
            _ => panic!("Unknown pair"),
        };

        let mut params = HashMap::new();
        params.insert("method", "balance");
        params.insert("pair", currency_pair);
        self.private_query(&params)
    }
}
