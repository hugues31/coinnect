//! Use this module to interact with Poloniex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Poloniex offers.

use serde_json::Value;
use serde_json::value::Map;

use exchange::ExchangeApi;
use poloniex::api::PoloniexApi;

use error::Error;
use pair::Pair;
use types::Ticker;
use poloniex::utils;
use helpers;

impl ExchangeApi for PoloniexApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker, Error> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(Error::PairUnsupported),
        };
        //let raw_response = self.get_ticker_information(&pair_name)?;

        //let result = utils::parse_result(raw_response)?;

        let price = 0.0;
        let ask = 0.0;
        let bid = 0.0;
        let vol = 0.0;

        Ok(Ticker {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            last_trade_price: price,
            lowest_ask: ask,
            highest_bid: bid,
            volume: Some(vol),
        })

    }
    fn return_trade_history(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_order_book(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_balances(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
}
