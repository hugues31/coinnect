//! Types definition used for handling returned data when generic API is used.

use pair::Pair;

type Price = f64;
type Volume = f64;

#[derive(Debug)]
pub struct Ticker {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// The Pair corresponding to the Ticker returned (maybe useful later for asynchronous APIs)
    pub pair: Pair,
    /// Last trade price found in the history
    pub last_trade_price: Price,
    /// Lowest ask price found in Orderbook
    pub lowest_ask: Price,
    /// Highest bid price found in Orderbook
    pub highest_bid: Price,
    // Bittrex does not support Volume for ticker so volume could be None
    /// Last 24 hours volume (quote-volume)
    pub volume: Option<Volume>,
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
