//! Use this module to interact with Bitstamp through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bitstamp offers.

use exchange::ExchangeApi;
use bitstamp::api::BitstampApi;

use error::Error;
use pair::Pair;
use types::{Ticker, Orderbook};
use helpers;

impl ExchangeApi for BitstampApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker, Error> {

        let result = self.return_ticker(pair)?;

        let price = result["last"].as_str().unwrap().parse::<f64>().unwrap();
        let ask = result["ask"].as_str().unwrap().parse::<f64>().unwrap();
        let bid = result["bid"].as_str().unwrap().parse::<f64>().unwrap();
        let vol = result["volume"].as_str().unwrap().parse::<f64>().unwrap();

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
