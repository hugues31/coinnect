//! Use this module to interact with Poloniex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Poloniex offers.

use serde_json::Value;
use serde_json::value::Map;

use exchange::ExchangeApi;
use poloniex::api::PoloniexApi;

use error::*;
use pair::Pair;
use types::Ticker;
use poloniex::utils;
use helpers;

impl ExchangeApi for PoloniexApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };
        let raw_response = self.return_ticker()?;

        let result = utils::parse_result(raw_response)?;

        let price =
            result[*pair_name]["last"]
                .as_str()
                .ok_or(ErrorKind::MissingField(format!("{}.last", pair_name)))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.last", pair_name)))?;
        let ask =
            result[*pair_name]["lowestAsk"]
                .as_str()
                .ok_or(ErrorKind::MissingField(format!("{}.lowestAsk", pair_name)))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.lowestAsk", pair_name)))?;
        let bid =
            result[*pair_name]["highestBid"]
                .as_str()
                .ok_or(ErrorKind::MissingField(format!("{}.hightestBid", pair_name)))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.highestBid", pair_name)))?;
        let vol =
            result[*pair_name]["quoteVolume"]
                .as_str()
                .ok_or(ErrorKind::MissingField(format!("{}.quoteVolume", pair_name)))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.quoteVolume", pair_name)))?;

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
