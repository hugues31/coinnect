//! Use this module to interact with Gdax exchange.
//! Please see examples for more informations.


use hyper::{Client, Uri, Request, Body, Method};
use hyper::header::{CONTENT_TYPE,USER_AGENT,HeaderName};

use hyper_tls::HttpsConnector;

use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;

use crate::coinnect::Credentials;
use crate::exchange::Exchange;

use crate::error::*;
use crate::helpers;
use crate::types::Pair;
use crate::gdax::utils;
use crate::types::*;
use hyper::client::HttpConnector;
use futures::Stream;
use futures::{FutureExt, TryFutureExt};
use futures::io::{AsyncReadExt, AsyncRead};
use std::convert::TryInto;
use futures::select;
use bytes::buf::BufExt as _;
use crate::helpers::json;

#[derive(Debug)]
pub struct GdaxApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    customer_id: String,
    http_client: Client<HttpsConnector<HttpConnector>>,
    burst: bool,
}


impl GdaxApi {
    /// Create a new GdaxApi by providing an API key & API secret
    pub fn new<C: Credentials>(creds: C) -> Result<GdaxApi> {
        if creds.exchange() != Exchange::Gdax {
            return Err(ErrorKind::InvalidConfigType(Exchange::Gdax, creds.exchange()).into());
        }

        let connector = HttpsConnector::new();
        let ssl = Client::builder().build::<_, hyper::Body>(connector);

        Ok(GdaxApi {
               last_request: 0,
               api_key: creds.get("api_key").unwrap_or_default(),
               api_secret: creds.get("api_secret").unwrap_or_default(),
               customer_id: creds.get("customer_id").unwrap_or_default(),
               http_client: ssl,
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
            let threshold: u64 = 334; // 3 requests/sec = 1/3*1000
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
        let string = utils::build_url(method, pair);
        let url: Uri = string.as_str().parse().map_err(|_e| ErrorKind::BadParse)?;

        self.block_or_continue();
        let req: Result<Request<Body>> = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header(USER_AGENT, "coinnect")
            .body(Body::empty())
            .map_err(|e| ErrorKind::ServiceUnavailable(e.to_string()).into());

        let req2 = req.unwrap();
        let buf = futures::executor::block_on(self.http_client.request(req2)
            .and_then(|resp| hyper::body::aggregate(resp.into_body())))?;

        self.last_request = helpers::get_unix_timestamp_ms();
        let reader = buf.reader();
        json::deserialize_json_r(reader)
    }

    ///
    ///
    /// #Examples
    ///
    /// ```json
    /// extern crate coinnect;
    /// use crate::coinnect::gdax::GdaxApi;
    /// let mut api = GdaxApi::new("", "");
    /// let  result = api.private_query("balance", "btcusd");
    /// assert_eq!(true, true);
    /// ```
    fn private_query(&mut self, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {

        let method: &str = params
            .get("method")
            .ok_or_else(|| "Missing \"method\" field.")?;
        let pair: &str = params.get("pair").ok_or_else(|| "Missing \"pair\" field.")?;
        let string = utils::build_url(method, pair);
        let url: Uri = string.as_str().parse().map_err(|_e| ErrorKind::BadParse)?;

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
        let req: Result<Request<Body>> = Request::builder()
            .method("POST")
            .uri(url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded".to_owned())
            .body(Body::from(post_data))
            .map_err(|e| ErrorKind::ServiceUnavailable(e.to_string()).into());
        let req2 = req.unwrap();
        let buf = futures::executor::block_on(self.http_client.request(req2).and_then(|resp| hyper::body::aggregate(resp.into_body())))?;
        self.last_request = helpers::get_unix_timestamp_ms();
        let reader = buf.reader();
        json::deserialize_json_r(reader)
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
mod gdax_api_tests {
    use super::*;

//    #[test]
//    fn should_block_or_not_block_when_enabled_or_disabled() {
//        let mut api = GdaxApi {
//            last_request: helpers::get_unix_timestamp_ms(),
//            api_key: "".to_string(),
//            api_secret: "".to_string(),
//            customer_id: "".to_string(),
//            http_client: Client::new(),
//            burst: false,
//        };
//
//        let mut counter = 0;
//        loop {
//            api.set_burst(false);
//            let start = helpers::get_unix_timestamp_ms();
//            api.block_or_continue();
//            api.last_request = helpers::get_unix_timestamp_ms();
//
//            let difference = api.last_request - start;
//            assert!(difference >= 334);
//            assert!(difference < 10000);
//
//
//            api.set_burst(true);
//            let start = helpers::get_unix_timestamp_ms();
//            api.block_or_continue();
//            api.last_request = helpers::get_unix_timestamp_ms();
//
//            let difference = api.last_request - start;
//            assert!(difference < 10);
//
//            counter = counter + 1;
//            if counter >= 3 { break; }
//        }
//    }
}
