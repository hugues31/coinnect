//! Use this module to interact with Kraken through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Kraken offers.

use serde_json::Value;
use serde_json::value::Map;

use exchange::ExchangeApi;
use kraken::api::KrakenApi;

use error::*;
use pair::Pair;
use types::Ticker;
use kraken::utils;
use helpers;

impl ExchangeApi for KrakenApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_ticker_information(&pair_name)?;

        let result = utils::parse_result(raw_response)?;

        let price = result[*pair_name]["c"][0]
            .as_str()
            .ok_or(ErrorKind::MissingField(format!("{}.c", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.c", pair_name)))?;
        let ask = result[*pair_name]["a"][0]
            .as_str()
            .ok_or(ErrorKind::MissingField(format!("{}.a", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.a", pair_name)))?;
        let bid = result[*pair_name]["b"][0]
            .as_str()
            .ok_or(ErrorKind::MissingField(format!("{}.b", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.b", pair_name)))?;
        let vol = result[*pair_name]["v"][1]
            .as_str()
            .ok_or(ErrorKind::MissingField(format!("{}.v", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.v", pair_name)))?;

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
