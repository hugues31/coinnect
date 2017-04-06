//! This module contains Exchange enum.

use serde_json::value::Map;
use serde_json::value::Value;

use std::collections::HashMap;
use std::fmt::Debug;

use error::Error;
use pair::Pair;
use types::TickerInfo;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
}

pub trait ExchangeApi: Debug {
    /// Return a TickerInfo for the Pair specified. If no Pair is specified (None),
    /// return all Tickers available.
    fn ticker(&mut self, pair: Option<Pair>) -> Result<TickerInfo, Error>;

    fn return_order_book(&mut self, pair: Pair) -> Option<Map<String, Value>>;
    fn return_trade_history(&mut self, pair: Pair) -> Option<Map<String, Value>>;
    fn return_balances(&mut self, pair: Pair) -> Option<Map<String, Value>>;
    // fn balances(&mut self, pair: Option<Asset>) -> Result<Vec<Asset, Volume>, Error>;
}
