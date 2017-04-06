//! Types definition used for handling returned data

use pair::Pair;

type Price = f64;
type Volume = f64;

pub struct TickerInfo {
    /// unix timestamp in ms (when the response was received)
    timestamp: i64,
    ticker: Vec<Ticker>,
}

pub struct Ticker {
    pair: Pair,
    last_trade_price: Price,
    lowest_ask: Price,
    highest_bid: Price,
    /// Last 24 hours volume
    volume: Option<Volume>, // Bittrex does not support Volume for ticker so volume could be None
}

/*
pub struct Orderbook {
    timestamp: i64,
    ask_offers: Vec<Price, Volume>,
    bid_offers: Vec<Price, Volume>,
}

impl Orderbook {
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_ask_offers(&self) -> Vec<Price, Volume> {
        self.ask_offers
    }

    pub fn get_bid_offers(&self) -> Vec<Price, Volume> {
        self.bid_offers
    }
}
*/
