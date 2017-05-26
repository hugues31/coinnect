//! This module contains Exchange enum.

use std::fmt::Debug;

use error::*;
use pair::Pair;
use types::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
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
