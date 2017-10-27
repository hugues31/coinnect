//! Use this module to interact with Bittrex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bittrex offers.

use bigdecimal::BigDecimal;
use std::str::FromStr;

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

        let raw_response = self.get_market_summary(pair_name)?;

        let result = utils::parse_result(&raw_response)?;
        let result_array = result.as_array();
        let result_obj = result_array.unwrap()[0].as_object().unwrap();

        let price_str = result_obj.get("Last").unwrap().as_f64().unwrap().to_string();
        let price = BigDecimal::from_str(&price_str).unwrap();

        let ask_str = result_obj.get("Ask").unwrap().as_f64().unwrap().to_string();
        let ask = BigDecimal::from_str(&price_str).unwrap();

        let bid_str = result_obj.get("Bid").unwrap().as_f64().unwrap().to_string();
        let bid = BigDecimal::from_str(&price_str).unwrap();

        let volume_str = result_obj.get("Volume").unwrap().as_f64().unwrap().to_string();
        let vol = BigDecimal::from_str(&price_str).unwrap();

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
