use serde_json::Value;
use serde_json::value::Map;

use exchange::ExchangeApi;
use kraken::api::KrakenApi;

use error::Error;
use pair::Pair;
use types::TickerInfo;
use kraken::utils;

impl ExchangeApi for KrakenApi {
    fn ticker(&mut self, pair: Option<Pair>) -> Result<TickerInfo, Error> {
        let pair_converted = match pair {
            Some(pair) => &utils::get_pair_string(pair).unwrap_or(return Err(Error::PairUnsupported)),
            None => "all",  // this is specific to Kraken API
        };
        let raw_response = self.get_ticker_information(pair_converted);
        unimplemented!();
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
