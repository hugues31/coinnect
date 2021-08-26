//! This module contains Exchange enum.

use std::fmt::Debug;
use std::convert::Into;
use std::str::FromStr;

use crate::error::*;
use crate::types::*;




#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
    Bittrex,
    Gdax,
}

impl Into<String> for Exchange {
    fn into(self) -> String {
        match self {
            Exchange::Bitstamp => "Bitstamp".to_string(),
            Exchange::Kraken => "Kraken".to_string(),
            Exchange::Poloniex => "Poloniex".to_string(),
            Exchange::Bittrex => "Bittrex".to_string(),
            Exchange::Gdax => "Gdax".to_string(),
        }
    }
}

impl FromStr for Exchange {
    type Err = Error;

    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "bitstamp" => Ok(Exchange::Bitstamp),
            "kraken" => Ok(Exchange::Kraken),
            "poloniex" => Ok(Exchange::Poloniex),
            "bittrex" => Ok(Exchange::Bittrex),
            "gdax" => Ok(Exchange::Gdax),
            _ => Err(ErrorKind::InvalidExchange(input.to_string()).into()),
        }
    }
}

pub trait ExchangeApi: Debug {
    /// Return a Ticker for the Pair specified.
    fn ticker(&mut self, pair: Pair) -> Result<Ticker>;

    /// Return an Orderbook for the specified Pair.
    fn orderbook(&mut self, pair: Pair) -> Result<Orderbook>;

    /// Place an order directly to the exchange.
    /// Quantity is in quote currency. So if you want to buy 1 Bitcoin for X€ (pair BTC_EUR),
    /// base currency (right member in the pair) is BTC and quote/counter currency is BTC (left
    /// member in the pair).
    /// So quantity = 1.
    ///
    /// A good practice is to store the return type (OrderInfo) somewhere since it can later be used
    /// to modify or cancel the order.
    fn add_order(&mut self,
                 order_type: OrderType,
                 pair: Pair,
                 quantity: Volume,
                 price: Option<Price>)
                 -> Result<OrderInfo>;

    /// Retrieve the current amounts of all the currencies that the account holds
    /// The amounts returned are available (not used to open an order)
    fn balances(&mut self) -> Result<Balances>;
}
