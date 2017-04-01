use exchange::{ Exchange, ExchangeApi };
use pair::Pair;
use bitstamp::api::BitstampApi;

use std::collections::HashMap;
use serde_json::value::Map;
use serde_json::value::Value;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Coinnect {
    BitstampApi
}

impl Coinnect {
    /// Create a new CoinnectApi by providing an API key & API secret
    pub fn new(exchange: Exchange, customer_id: &str, api_key: &str, api_secret: &str) -> Box<ExchangeApi> {

        let mut params = HashMap::new();
        params.insert("customer_id", customer_id);
        params.insert("api_key", api_key);
        params.insert("api_secret", api_secret);

        match exchange {
            Exchange::Bitstamp => Box::new(BitstampApi::new(&params)),
            Exchange::Kraken => Box::new(UnimplementedApi),
            Exchange::Poloniex => Box::new(UnimplementedApi),
        }
    }
}


#[derive(Debug)]
struct UnimplementedApi;

impl ExchangeApi for UnimplementedApi {
    fn public_query(&mut self, _: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        panic!("Not implemented");
    }
    fn private_query(&mut self, _: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        panic!("Not implemented");
    }

    fn return_ticker(&mut self, pair: Pair) -> Option<Map<String, Value>> {
        panic!("Not implemented");
    }
    fn return_order_book(&mut self, _: &str) -> Option<Map<String, Value>> {
        panic!("Not implemented");
    }
    fn return_balances(&mut self, _: &str) -> Option<Map<String, Value>> {
        panic!("Not implemented");
    }
}
