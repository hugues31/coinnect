//! Use this module to interact with Bitstamp through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bitstamp offers.

use serde_json::Value;
use serde_json::value::Map;

use exchange::ExchangeApi;
use bitstamp::api::BitstampApi;

use error::*;
use pair::Pair;
use types::Ticker;
use helpers;

impl ExchangeApi for BitstampApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {

        let result = self.return_ticker(pair)?;

        //let parse_as_float = |field: &str| field.parse::<f64>()?;

        let price = result["last"]
            .as_str()
            .ok_or(ErrorKind::MissingField("last".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("last".to_string()))?;

        let ask = result["ask"]
            .as_str()
            .ok_or(ErrorKind::MissingField("ask".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("ask".to_string()))?;
        let bid = result["bid"]
            .as_str()
            .ok_or(ErrorKind::MissingField("bid".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("bid".to_string()))?;
        let vol = result["volume"]
            .as_str()
            .ok_or(ErrorKind::MissingField("volume".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("volume".to_string()))?;

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
