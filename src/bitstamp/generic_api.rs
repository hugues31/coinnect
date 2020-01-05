//! Use this module to interact with Bitstamp through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bitstamp offers.

use crate::exchange::ExchangeApi;
use crate::bitstamp::api::BitstampApi;
use crate::bitstamp::utils;

use crate::error::*;
use crate::types::*;
use crate::helpers;
use std::net::TcpStream;
use url::Url;
use log::*;
use futures::{Future, Stream};
use futures_util::TryFutureExt;
use futures::stream::{SplitSink, StreamExt, SplitStream, TryStreamExt, TryStream};
use awc::error::WsClientError;
use actix_codec::Framed;
use awc::BoxedSocket;
use awc::ws::{Codec, Frame};

impl ExchangeApi for BitstampApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {

        let result = self.return_ticker(pair)?;

        let price = helpers::from_json_bigdecimal(&result["last"], "last")?;
        let ask = helpers::from_json_bigdecimal(&result["ask"], "ask")?;
        let bid = helpers::from_json_bigdecimal(&result["bid"], "bid")?;
        let vol = helpers::from_json_bigdecimal(&result["volume"], "volume")?;

        Ok(Ticker {
               timestamp: helpers::get_unix_timestamp_ms(),
               pair: pair,
               last_trade_price: price,
               lowest_ask: ask,
               highest_bid: bid,
               volume: Some(vol),
           })
    }

    fn orderbook(&mut self, pair: Pair) -> Result<Orderbook> {

        let raw_response = self.return_order_book(pair)?;

        let result = utils::parse_result(&raw_response)?;

        let mut ask_offers = Vec::new();
        let mut bid_offers = Vec::new();

        let ask_array =
            result["asks"]
                .as_array()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["asks"])))?;
        let bid_array =
            result["bids"]
                .as_array()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["asks"])))?;

        for ask in ask_array {
            let price = helpers::from_json_bigdecimal(&ask[0], "ask price")?;
            let volume = helpers::from_json_bigdecimal(&ask[1], "ask volume")?;

            ask_offers.push((price, volume));
        }

        for bid in bid_array {
            let price = helpers::from_json_bigdecimal(&bid[0], "bid price")?;
            let volume = helpers::from_json_bigdecimal(&bid[1], "bid volume")?;

            bid_offers.push((price, volume));
        }

        Ok(Orderbook {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            asks: ask_offers,
            bids: bid_offers,
        })
    }

    fn add_order(&mut self,
                 order_type: OrderType,
                 pair: Pair,
                 quantity: Volume,
                 price: Option<Price>)
                 -> Result<OrderInfo> {
        //let pair_name = match utils::get_pair_string(&pair) {
        //Some(name) => name,
        //None => return Err(ErrorKind::PairUnsupported.into()),
        //};

        let result = match order_type {
            OrderType::BuyLimit => {
                if price.is_none() {
                    return Err(ErrorKind::MissingPrice.into());
                }

                // Unwrap safe here with the check above.
                self.buy_limit(pair, quantity, price.unwrap(), None, None)
            }
            OrderType::BuyMarket => self.buy_market(pair, quantity),
            OrderType::SellLimit => {
                if price.is_none() {
                    return Err(ErrorKind::MissingPrice.into());
                }

                // Unwrap safe here with the check above.
                self.sell_limit(pair, quantity, price.unwrap(), None, None)
            }
            OrderType::SellMarket => self.sell_market(pair, quantity),
        };

        Ok(OrderInfo {
               timestamp: helpers::get_unix_timestamp_ms(),
               identifier: vec![result?["id"]
                                    .as_str()
                                    .ok_or_else(|| {
                                                    ErrorKind::MissingField("id".to_string())
                                                })?
                                    .to_string()],
           })
    }

    /// Return the balances for each currency on the account
    fn balances(&mut self) -> Result<Balances> {
        let raw_response = self.return_balances()?;
        let result = utils::parse_result(&raw_response)?;

        let mut balances = Balances::new();

        for (key, val) in result.iter() {
            let currency = utils::get_currency_enum(key);

            match currency {
                Some(c) => {
                    let amount = helpers::from_json_bigdecimal(&val, "amount")?;

                    balances.insert(c, amount);
                },
                _ => ()
            }
        }

        Ok(balances)
    }
}
