//! Use this module to interact with Poloniex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Poloniex offers.

use exchange::ExchangeApi;
use poloniex::api::PoloniexApi;

use error::Error;
use pair::Pair;
use types::{Ticker, Orderbook};
use poloniex::utils;
use helpers;

impl ExchangeApi for PoloniexApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker, Error> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(Error::PairUnsupported),
        };
        let raw_response = self.return_ticker()?;

        let result = utils::parse_result(raw_response)?;

        let price = result[*pair_name]["last"].as_str().unwrap().parse::<f64>().unwrap();
        let ask = result[*pair_name]["lowestAsk"].as_str().unwrap().parse::<f64>().unwrap();
        let bid = result[*pair_name]["highestBid"].as_str().unwrap().parse::<f64>().unwrap();
        let vol = result[*pair_name]["quoteVolume"].as_str().unwrap().parse::<f64>().unwrap();

        Ok(Ticker {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            last_trade_price: price,
            lowest_ask: ask,
            highest_bid: bid,
            volume: Some(vol),
        })
    }

    fn orderbook(&mut self, pair: Pair) -> Result<Orderbook, Error> {
        unimplemented!();
    }
}
