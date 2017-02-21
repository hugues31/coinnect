//! Use this module to interact with Kraken exchange.
//! Please see examples for more informations.

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::{Sha256, Sha512};


use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;

use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;
use std::str;

use helpers;

header! { (KeyHeader, "API-Key") => [String] }
header! { (SignHeader, "API-Sign") => [String] }


pub struct KrakenApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
}


impl KrakenApi {
    /// Create a new KrakenApi by providing an API key & API secret
    pub fn new(api_key: &str, api_secret: &str) -> KrakenApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        KrakenApi {
            last_request: 0,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: Client::with_connector(connector),
        }
    }

    /// Create a new KrakenApi from a json configuration file. This file must follow this structure:
    ///
    /// ```ignore
    /// {
    ///     "account_kraken": {
    ///         "exchange"  : "kraken",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef"
    ///     },
    ///     "account_poloniex": {
    ///         "exchange"  : "poloniex",
    ///         "api_key"   : "XYXY-XYXY-XYXY-XY",
    ///         "api_secret": "A0A0B1B1C2C2"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Kraken account with
    /// `new_from_file("account_kraken", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> KrakenApi {
        let mut f = File::open(&path).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();
        let json_obj = data.as_object().unwrap().get(config_name).unwrap();
        let api_key = json_obj.get("api_key").unwrap().as_str().unwrap();
        let api_secret = json_obj.get("api_secret").unwrap().as_str().unwrap();

        KrakenApi::new(api_key, api_secret)
    }

    fn block_or_continue(&self) {
        let threshold = 2000; // 1 request/2sec
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

    fn public_query(&mut self, method: &str, params: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        let url = "https://api.kraken.com/0/public/".to_string() + method + "?" +
        &helpers::url_encode_hashmap(params);

        self.block_or_continue();
        let mut response = self.http_client.get(&url).send().unwrap();
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return self.deserialize_json(buffer);
    }

    fn private_query(&mut self, method: &str, params: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        let url = "https://api.kraken.com/0/private/".to_string() + method;

        let urlpath = "/0/private/".to_string() + method;

        // Create header
        let nonce = helpers::get_unix_timestamp_ms().to_string();
        let mut params = params.clone();

        params.insert("nonce", &nonce);

        let postdata = helpers::url_encode_hashmap(&params);
        let signature = self.create_signature(urlpath, &postdata, &nonce);

        let mut custom_header = header::Headers::new();
        custom_header.set(KeyHeader(self.api_key.clone()));
        custom_header.set(SignHeader(signature));

        let mut res = self.http_client
        .post(&url)
        .body(&postdata)
        .headers(custom_header)
        .send()
        .unwrap();

        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        return self.deserialize_json(buffer);
    }

    fn create_signature(&self, urlpath: String, postdata: &str, nonce: &str) -> String {
        let message_presha256 = nonce.to_string() + postdata;
        println!("message {:?}", message_presha256);

        let mut sha256 = Sha256::new();
        sha256.input_str(&message_presha256);
        let final_sha = sha256.result_str();

        let message_premac = urlpath + &final_sha;

        let hmac_key = self.api_secret.from_base64().unwrap();
        let mut hmac = Hmac::new(Sha512::new(), &hmac_key);
        hmac.input(message_premac.as_bytes());
        hmac.result().code().to_base64(STANDARD)
    }

    /// Result: Server's time
    ///
    /// ```ignore
    /// unixtime =  as unix timestamp
    /// rfc1123 = as RFC 1123 time format
    /// ```
    /// Note: This is to aid in approximating the skew time between the server and client.
    pub fn get_server_time(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("Time", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// info = info to retrieve (optional):
    ///     info = all info (default)
    /// aclass = asset class (optional):
    ///     currency (default)
    /// asset = comma delimited list of assets to get info on (optional.  default = all for given asset class)
    /// ```
    /// Result: array of asset names and their info:
    ///
    /// ```ignore
    /// <asset_name> = asset name
    /// altname = alternate name
    /// aclass = asset class
    /// decimals = scaling decimal places for record keeping
    /// display_decimals = scaling decimal places for output display
    /// ```
    pub fn get_asset_info(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("Assets", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// info = info to retrieve (optional):
    ///     info = all info (default)
    ///     leverage = leverage info
    ///     fees = fees schedule
    ///     margin = margin info
    /// pair = comma delimited list of asset pairs to get info on (optional.  default = all)
    /// ```
    ///
    /// Result: array of pair names and their info
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     altname = alternate pair name
    ///     aclass_base = asset class of base component
    ///     base = asset id of base component
    ///     aclass_quote = asset class of quote component
    ///     quote = asset id of quote component
    ///     lot = volume lot size
    ///     pair_decimals = scaling decimal places for pair
    ///     lot_decimals = scaling decimal places for volume
    ///     lot_multiplier = amount to multiply lot volume by to get currency volume
    ///     leverage_buy = array of leverage amounts available when buying
    ///     leverage_sell = array of leverage amounts available when selling
    ///     fees = fee schedule array in [volume, percent fee] tuples
    ///     fees_maker = maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    ///     fee_volume_currency = volume discount currency
    ///     margin_call = margin call level
    ///     margin_stop = stop-out/liquidation margin level
    /// ```
    pub fn get_tradable_asset_pairs(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("AssetPairs", &params)
    }

    /// Input: pair = comma delimited list of asset pairs to get info on
    /// Result: array of pair names and their ticker info
    ///
    /// ```ignore
    /// <pair_name> = pair name
    /// a = ask array(<price>, <whole lot volume>, <lot volume>),
    /// b = bid array(<price>, <whole lot volume>, <lot volume>),
    /// c = last trade closed array(<price>, <lot volume>),
    /// v = volume array(<today>, <last 24 hours>),
    /// p = volume weighted average price array(<today>, <last 24 hours>),
    /// t = number of trades array(<today>, <last 24 hours>),
    /// l = low array(<today>, <last 24 hours>),
    /// h = high array(<today>, <last 24 hours>),
    /// o = today's opening price
    /// ```
    pub fn get_ticker_information(&mut self, pair: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        self.public_query("Ticker", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get OHLC data for
    /// interval = time frame interval in minutes (optional):
    /// 	1 (default), 5, 15, 30, 60, 240, 1440, 10080, 21600
    /// since = return committed OHLC data since given id (optional.  exclusive)
    /// ```
    ///
    /// Result: array of pair name and OHLC data
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     array of array entries(<time>, <open>, <high>, <low>, <close>, <vwap>, <volume>, <count>)
    /// last = id to be used as since when polling for new, committed OHLC data
    /// ```
    ///
    /// Note: the last entry in the OHLC array is for the current, not-yet-committed frame and will always be present, regardless of the value of "since".
    pub fn get_ohlc_data(&mut self, pair: &str, interval: &str, since: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("interval", interval);
        params.insert("since", since);
        self.public_query("OHLC", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get market depth for
    /// count = maximum number of asks/bids (optional)
    /// ```
    /// Result: array of pair name and market depth
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     asks = ask side array of array entries(<price>, <volume>, <timestamp>)
    ///     bids = bid side array of array entries(<price>, <volume>, <timestamp>)
    /// ```
    pub fn get_order_book(&mut self, pair: &str, count: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("count", count);
        self.public_query("Depth", &params)
    }


    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get trade data for
    /// since = return trade data since given id (optional.  exclusive)
    /// ```
    /// Result: array of pair name and recent trade data
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     array of array entries(<price>, <volume>, <time>, <buy/sell>, <market/limit>, <miscellaneous>)
    /// last = id to be used as since when polling for new trade data
    /// ```
    pub fn get_recent_trades(&mut self, pair: &str, since: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("since", since);
        self.public_query("Trades", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get spread data for
    /// since = return spread data since given id (optional.  inclusive)
    /// ```
    ///
    /// Result: array of pair name and recent spread data
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     array of array entries(<time>, <bid>, <ask>)
    /// last = id to be used as since when polling for new spread data
    /// ```
    /// Note: "since" is inclusive so any returned data with the same time as the previous set should overwrite all of the previous set's entries at that time
    pub fn get_recent_spread_data(&mut self, pair: &str, since: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("since", since);
        self.public_query("Spread", &params)
    }

    /// Result: array of asset names and balance amount
    pub fn get_account_balance(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("Balance", &params)
    }
}
