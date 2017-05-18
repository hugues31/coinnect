//! Use this module to interact with Kraken through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Kraken offers.

use exchange::ExchangeApi;
use kraken::api::KrakenApi;

use error::Error;
use pair::Pair;
use types::{Ticker, Orderbook};
use kraken::utils;
use helpers;

impl ExchangeApi for KrakenApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker, Error> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(Error::PairUnsupported),
        };

        let raw_response = self.get_ticker_information(&pair_name)?;

        let result = utils::parse_result(raw_response)?;

        let price = result[*pair_name]["c"][0].as_str().unwrap().parse::<f64>().unwrap();
        let ask = result[*pair_name]["a"][0].as_str().unwrap().parse::<f64>().unwrap();
        let bid = result[*pair_name]["b"][0].as_str().unwrap().parse::<f64>().unwrap();
        let vol = result[*pair_name]["v"][1].as_str().unwrap().parse::<f64>().unwrap();

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
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(Error::PairUnsupported),
        };

        let raw_response = self.get_order_book(&pair_name, "1000")?;    // 1000 entries max

        let result = utils::parse_result(raw_response)?;

        let mut ask_offers = Vec::new();
        let mut bid_offers = Vec::new();

        let ask_array = result[*pair_name]["asks"].as_array().unwrap();
        let bid_array = result[*pair_name]["bids"].as_array().unwrap();

        for ask in ask_array {
            let price = ask[0].as_str().unwrap().parse::<f64>().unwrap();
            let volume = ask[1].as_str().unwrap().parse::<f64>().unwrap();
            ask_offers.push((price, volume));
        }

        for bid in bid_array {
            let price = bid[0].as_str().unwrap().parse::<f64>().unwrap();
            let volume = bid[1].as_str().unwrap().parse::<f64>().unwrap();
            bid_offers.push((price, volume));
        }

        Ok(Orderbook {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            asks: ask_offers,
            bids: bid_offers,
        })
    }
}
