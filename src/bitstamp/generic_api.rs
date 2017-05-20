//! Use this module to interact with Bitstamp through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bitstamp offers.

use exchange::ExchangeApi;
use bitstamp::api::BitstampApi;
use bitstamp::utils;
use error::Error;
use pair::Pair;
use types::*;
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

    fn add_order(&mut self,
                 order_type: OrderType,
                 pair: Pair,
                 quantity: Volume,
                 price: Option<Price>)
                 -> Result<OrderInfo, Error> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(Error::PairUnsupported),
        };

        let result = match order_type {
            OrderType::BuyLimit => self.buy_limit(pair, quantity, price.unwrap(), None, None),
            OrderType::BuyMarket => self.buy_market(pair, quantity),
            OrderType::SellLimit => self.sell_limit(pair, quantity, price.unwrap(), None, None),
            OrderType::SellMarket => self.sell_market(pair, quantity),
        };

        Ok(OrderInfo {
            timestamp: helpers::get_unix_timestamp_ms(),
            identifier: vec![result.unwrap()["id"].as_str().unwrap().to_string()],
        })
    }
}
