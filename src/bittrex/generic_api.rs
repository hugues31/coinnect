//! Use this module to interact with Bittrex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bittrex offers.

use exchange::ExchangeApi;
use bittrex::api::BittrexApi;

use error::*;
use types::*;
use bittrex::utils;
use helpers;

impl ExchangeApi for BittrexApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        unimplemented!();

    }

    fn orderbook(&mut self, pair: Pair) -> Result<Orderbook> {
        unimplemented!();
    }

    fn add_order(&mut self,
                 order_type: OrderType,
                 pair: Pair,
                 quantity: Volume,
                 price: Option<Price>)
                 -> Result<OrderInfo> {
        unimplemented!();
    }

    fn balances(&mut self) -> Result<Balances> {
        unimplemented!();
    }
}
