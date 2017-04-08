//! This module contains Exchange enum.

use serde_json::value::Map;
use serde_json::value::Value;

use std::fmt::Debug;

use error::Error;
use pair::Pair;
use types::Ticker;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
}

pub trait ExchangeApi: Debug {
    /// Return a Ticker for the Pair specified.
    fn ticker(&mut self, pair: Pair) -> Result<Ticker, Error>;

    fn return_order_book(&mut self, pair: Pair) -> Option<Map<String, Value>>;
    fn return_trade_history(&mut self, pair: Pair) -> Option<Map<String, Value>>;
    fn return_balances(&mut self, pair: Pair) -> Option<Map<String, Value>>;
    // fn balances(&mut self, pair: Option<Asset>) -> Result<Vec<Asset, Volume>, Error>;
}
