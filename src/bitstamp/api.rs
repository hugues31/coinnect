//! Use this module to interact with Bitstamp exchange.
//! Please see examples for more informations.


use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header::ContentType;
use hyper::net::HttpsConnector;

use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;

use coinnect::Credentials;
use exchange::Exchange;

use error::*;
use helpers;
use types::Pair;
use bitstamp::utils;
use types::*;

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
    burst: bool,
}


impl BitstampApi {
    /// Create a new BitstampApi by providing an API key & API secret
    pub fn new<C: Credentials>(creds: C) -> Result<BitstampApi> {
        if creds.exchange() != Exchange::Bitstamp {
            return Err(ErrorKind::InvalidConfigType(Exchange::Bitstamp, creds.exchange()).into());
        }

        //TODO: Handle correctly TLS errors with error_chain.
        let ssl = match NativeTlsClient::new() {
            Ok(res) => res,
            Err(_) => return Err(ErrorKind::TlsError.into()),
        };

        let connector = HttpsConnector::new(ssl);


        Ok(BitstampApi {
               last_request: 0,
               api_key: creds.get("api_key").unwrap_or_default(),
               api_secret: creds.get("api_secret").unwrap_or_default(),
               customer_id: creds.get("customer_id").unwrap_or_default(),
               http_client: Client::with_connector(connector),
               burst: false, // No burst by default
           })
    }

    /// The number of calls in a given period is limited. In order to avoid a ban we limit
    /// by default the number of api requests.
    /// This function sets or removes the limitation.
    /// Burst false implies no block.
    /// Burst true implies there is a control over the number of calls allowed to the exchange
    pub fn set_burst(&mut self, burst: bool) {
        self.burst = burst
    }

    fn block_or_continue(&self) {
        if ! self.burst {
            let threshold: u64 = 1000; // 600 requests per 10 mins = 1 request per second
            let offset: u64 = helpers::get_unix_timestamp_ms() as u64 - self.last_request as u64;
            if offset < threshold {
                let wait_ms = Duration::from_millis(threshold - offset);
                thread::sleep(wait_ms);
            }
        }
    }

    fn public_query(&mut self, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {

        let method: &str = params
            .get("method")
            .ok_or_else(|| "Missing \"method\" field.")?;
        let pair: &str = params.get("pair").ok_or_else(|| "Missing \"pair\" field.")?;
        let url: String = utils::build_url(method, pair);

        self.block_or_continue();
        let mut response = self.http_client.get(&url).send()?;
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        utils::deserialize_json(&buffer)
    }

    ///
    ///
    /// #Examples
    ///
    /// ```json
    /// extern crate coinnect;
    /// use coinnect::bitstamp::BitstampApi;
    /// let mut api = BitstampApi::new("", "");
    /// let  result = api.private_query("balance", "btcusd");
    /// assert_eq!(true, true);
    /// ```
    fn private_query(&mut self, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {

        let method: &str = params
            .get("method")
            .ok_or_else(|| "Missing \"method\" field.")?;
        let pair: &str = params.get("pair").ok_or_else(|| "Missing \"pair\" field.")?;
        let url: String = utils::build_url(method, pair);

        let nonce = utils::generate_nonce(None);
        let signature =
            utils::build_signature(&nonce, &self.customer_id, &self.api_key, &self.api_secret)?;

        let copy_api_key = self.api_key.clone();
        let mut post_params: &mut HashMap<&str, &str> = &mut HashMap::new();
        post_params.insert("key", &copy_api_key);
        post_params.insert("signature", &signature);
        post_params.insert("nonce", &nonce);

        // copy params into post_params .... bit of a hack but will do for now
        params.iter().for_each(|(k,v)| {
            post_params.insert(k,v);
        });

        helpers::strip_empties(&mut post_params);
        let post_data = helpers::url_encode_hashmap(post_params);
        let mut response = self.http_client
            .post(&url)
            .header(ContentType::form_url_encoded())
            .body(&post_data)
            .send()?;

        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        utils::deserialize_json(&buffer)
    }

    /// Sample output :
    ///
    /// ```json
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
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("pair", pair_name);
        params.insert("method", "ticker");
        self.public_query(&params)
    }

    /// Sample output :
    ///
    /// ```json
    /// {"asks":[[0.00007600,1164],[0.00007620,1300], ... ], "bids":[[0.00006901,200],
    /// [0.00006900,408], ... ], "timestamp": "1234567890"}
    /// ```
    pub fn return_order_book(&mut self, pair: Pair) -> Result<Map<String, Value>> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),

        };

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("method", "order_book");
        params.insert("pair", pair_name);
        self.public_query(&params)
    }

    /// Sample output :
    ///
    /// ```json
    /// [{"date":"2014-02-10 04:23:23","type":"buy","rate":"0.00007600","amount":"140",
    /// "total":"0.01064"},
    /// {"date":"2014-02-10 01:19:37","type":"buy","rate":"0.00007600","amount":"655",
    /// "total":"0.04978"}, ... ]
    /// ```
    pub fn return_trade_history(&mut self, pair: Pair) -> Result<Map<String, Value>> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("pair", pair_name);
        params.insert("method", "transactions");
        self.public_query(&params)
    }


    /// Returns all of your available balances.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"BTC":"0.59098578","LTC":"3.31117268", ... }
    /// ```
    pub fn return_balances(&mut self) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("method", "balance");
        params.insert("pair", "");
        self.private_query(&params)
    }

    /// Add a buy limit order to the exchange
    /// limit_price	: If the order gets executed, a new sell order will be placed,
    /// with "limit_price" as its price.
    /// daily_order (Optional) : Opens buy limit order which will be canceled
    /// at 0:00 UTC unless it already has been executed. Possible value: True
    pub fn buy_limit(&mut self,
                     pair: Pair,
                     amount: Volume,
                     price: Price,
                     price_limit: Option<Price>,
                     daily_order: Option<bool>)
                     -> Result<Map<String, Value>> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let amount_string = amount.to_string();
        let price_string = price.to_string();
        let price_limit_string = match price_limit {
            Some(limit) => limit.to_string(),
            None => "".to_string(),
        };

        let mut params = HashMap::new();
        params.insert("method", "buy");
        params.insert("pair", pair_name);

        params.insert("amount", &amount_string);
        params.insert("price", &price_string);
        params.insert("limit_price", &price_limit_string);
        if let Some(order) = daily_order {
            let daily_order_str = if order { "True" } else { "" }; // False is not a possible value
            params.insert("daily_order", daily_order_str);
        }

        self.private_query(&params)
    }

    /// Add a sell limit order to the exchange
    /// limit_price	: If the order gets executed, a new sell order will be placed,
    /// with "limit_price" as its price.
    /// daily_order (Optional) : Opens sell limit order which will be canceled
    /// at 0:00 UTC unless it already has been executed. Possible value: True
    pub fn sell_limit(&mut self,
                      pair: Pair,
                      amount: Volume,
                      price: Price,
                      price_limit: Option<Price>,
                      daily_order: Option<bool>)
                      -> Result<Map<String, Value>> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let amount_string = amount.to_string();
        let price_string = price.to_string();
        let price_limit_string = match price_limit {
            Some(limit) => limit.to_string(),
            None => "".to_string(),
        };

        let mut params = HashMap::new();
        params.insert("method", "sell");
        params.insert("pair", pair_name);

        params.insert("amount", &amount_string);
        params.insert("price", &price_string);
        params.insert("limit_price", &price_limit_string);
        if let Some(order) = daily_order {
            let daily_order_str = if order { "True" } else { "" }; // False is not a possible value
            params.insert("daily_order", daily_order_str);
        }

        self.private_query(&params)
    }

    /// Add a market buy order to the exchange
    /// By placing a market order you acknowledge that the execution of your order depends
    /// on the market conditions and that these conditions may be subject to sudden changes
    /// that cannot be foreseen.
    pub fn buy_market(&mut self, pair: Pair, amount: Volume) -> Result<Map<String, Value>> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let amount_string = amount.to_string();

        let mut params = HashMap::new();
        params.insert("method", "buy/market");
        params.insert("pair", pair_name);

        params.insert("amount", &amount_string);

        self.private_query(&params)
    }

    /// Add a market sell order to the exchange
    /// By placing a market order you acknowledge that the execution of your order depends
    /// on the market conditions and that these conditions may be subject to sudden changes
    /// that cannot be foreseen.
    pub fn sell_market(&mut self, pair: Pair, amount: Volume) -> Result<Map<String, Value>> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let amount_string = amount.to_string();

        let mut params = HashMap::new();
        params.insert("method", "sell/market");
        params.insert("pair", pair_name);

        params.insert("amount", &amount_string);

        self.private_query(&params)
    }
}


#[cfg(test)]
mod bitstamp_api_tests {
    use super::*;

    #[test]
    fn should_block_or_not_block_when_enabled_or_disabled() {
        let mut api = BitstampApi {
            last_request: helpers::get_unix_timestamp_ms(),
            api_key: "".to_string(),
            api_secret: "".to_string(),
            customer_id: "".to_string(),
            http_client: Client::new(),
            burst: false,
        };

        let mut counter = 0;
        loop {
            api.set_burst(false);
            let start = helpers::get_unix_timestamp_ms();
            api.block_or_continue();
            api.last_request = helpers::get_unix_timestamp_ms();

            let difference = api.last_request - start;
            assert!(difference >= 999);
            assert!(difference < 10000);


            api.set_burst(true);
            let start = helpers::get_unix_timestamp_ms();
            api.block_or_continue();
            api.last_request = helpers::get_unix_timestamp_ms();

            let difference = api.last_request - start;
            assert!(difference < 10);

            counter = counter + 1;
            if counter >= 3 { break; }
        }
    }
}
