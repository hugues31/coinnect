//! Use this module to interact with Bitstamp exchange.
//! Please see examples for more informations.

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::net::HttpsConnector;

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;

use helpers;


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


pub struct BitstampApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
}


impl BitstampApi {
    /// Create a new BitstampApi by providing an API key & API secret
    pub fn new(api_key: &str, api_secret: &str) -> BitstampApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        BitstampApi {
            last_request: 0,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: Client::with_connector(connector),
        }
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
    ///         "exchange"  : "bitstamp",
    ///         "api_key"   : "XYXY-XYXY-XYXY-XY",
    ///         "api_secret": "A0A0B1B1C2C2"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file("account_bitstamp", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> BitstampApi {
        let mut f = File::open(&path).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();
        let json_obj = data.as_object().unwrap().get(config_name).unwrap();
        let api_key = json_obj.get("api_key").unwrap().as_str().unwrap();
        let api_secret = json_obj.get("api_secret").unwrap().as_str().unwrap();

        BitstampApi::new(api_key, api_secret)
    }

    fn block_or_continue(&self) {
        let threshold = 1000; // 600 requests per 10 mins = 1 request per second
        let delay = helpers::get_unix_timestamp_ms() - self.last_request;
        if delay < threshold {
            let duration_ms = Duration::from_millis(delay as u64);
            thread::sleep(duration_ms);
        }
    }

    fn deserialize_json(&mut self, json_string: String) -> Option<Map<String, Value>> {
        let data: Value = serde_json::from_str(&json_string).unwrap();

        match data.as_object() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn build_url(method: &str, pair: &str) -> String {
        "https://www.bitstamp.net/api/v2/".to_string() + method + "/" + &pair + "/"
    }

    fn public_query(&mut self,
                    params: &HashMap<&str, &str>)
                    -> Option<Map<String, Value>> {
        let mut params = params.clone();
        helpers::strip_empties(&mut params);

        // TODO pass the pair as parameter, dont hardcode it #FIXME
        let method: &str = params.get("method").unwrap();
        let pair: &str = params.get("pair").unwrap();
        let url: String = BitstampApi::build_url(method, pair);

        self.block_or_continue();
        let mut response = self.http_client.get(&url).send().unwrap();
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return self.deserialize_json(buffer);
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
    pub fn return_ticker(&mut self) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("method", "ticker");
        params.insert("pair", "btcusd");
        self.public_query(&params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"asks":[[0.00007600,1164],[0.00007620,1300], ... ], "bids":[[0.00006901,200],
    /// [0.00006900,408], ... ], "timestamp": 1234567890"}
    /// ```
    pub fn return_order_book(&mut self,
                             pair: &str)
                             -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("method", "order_book");
        params.insert("pair", pair);
        self.public_query(&params)
    }
}
