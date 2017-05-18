//! This module contains Exchange enum.

use std::fmt::Debug;

use error::Error;
use pair::Pair;
use types::{Ticker, Orderbook};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
}

pub trait ExchangeApi: Debug {
    /// Return a Ticker for the specified Pair.
    fn ticker(&mut self, pair: Pair) -> Result<Ticker, Error>;

    /// Return an Orderbook for the specified Pair.
    fn orderbook(&mut self, pair: Pair) -> Result<Orderbook, Error>;
}
