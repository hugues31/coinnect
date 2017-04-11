//! Use this module to create a generic API.



use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::value::Map;
use serde_json::value::Value;

use error::Error;
use types::Ticker;
use exchange::{Exchange, ExchangeApi};
use pair::Pair;
use bitstamp::api::BitstampApi;
use kraken::api::KrakenApi;
use poloniex::api::PoloniexApi;

#[derive(Debug)]
pub struct Coinnect;

impl Coinnect {
    /// Create a new CoinnectApi by providing an API key & API secret
    pub fn new(exchange: Exchange,
               api_key: &str,
               api_secret: &str,
               customer_id: Option<&str>)
               -> Box<ExchangeApi> {
        match exchange {
            Exchange::Bitstamp => {
                let mut params = HashMap::new();
                params.insert("api_key", api_key);
                params.insert("api_secret", api_secret);
                if customer_id.is_some() {
                    params.insert("customer_id", customer_id.unwrap());
                }
                Box::new(BitstampApi::new(&params))
            }

            Exchange::Kraken => Box::new(KrakenApi::new(api_key, api_secret)),

            Exchange::Poloniex => Box::new(PoloniexApi::new(api_key, api_secret)),
        }
    }

    /// Create a new CoinnectApi from a json configuration file. This file must follow this
    /// structure:
    ///
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file(Exchange::Bitstamp, "account_bitstamp", Path::new("/keys.json"))`
    pub fn new_from_file(exchange: Exchange, config_name: &str, path: PathBuf) -> Box<ExchangeApi> {
        match exchange {
            Exchange::Bitstamp => Box::new(BitstampApi::new_from_file(config_name, path)),
            Exchange::Kraken => Box::new(UnimplementedApi),
            Exchange::Poloniex => Box::new(UnimplementedApi),
        }
    }
}


#[derive(Debug)]
struct UnimplementedApi;

impl ExchangeApi for UnimplementedApi {
    fn ticker(&mut self, _pair: Pair) -> Result<Ticker, Error> {
        unimplemented!();
    }
    fn return_trade_history(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_order_book(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_balances(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
}
