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
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_ticker(pair_name)?;

        Ok(Ticker {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            last_trade_price: price,
            lowest_ask: ask,
            highest_bid: bid,
            volume: Some(vol),
        })

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
