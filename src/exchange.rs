use serde_json::value::Map;
use serde_json::value::Value;

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
}

pub trait ExchangeApi<T> {
    fn new(params: &HashMap<&str, &str>) -> T;
    fn new_from_file(config_name: &str, path: PathBuf) -> T;

    fn public_query(&mut self, params: &HashMap<&str, &str>) -> Option<Map<String, Value>>;
    fn private_query(&mut self, params: &HashMap<&str, &str>) -> Option<Map<String, Value>>;

    fn return_ticker(&mut self) -> Option<Map<String, Value>>;
    fn return_order_book(&mut self, pair: &str) -> Option<Map<String, Value>>;
    fn return_balances(&mut self, pair: &str) -> Option<Map<String, Value>>;
}
