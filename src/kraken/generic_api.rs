//! Use this module to interact with Kraken through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Kraken offers.

use crate::exchange::ExchangeApi;
use crate::kraken::api::KrakenApi;

use crate::error::*;
use crate::types::*;
use crate::kraken::utils;
use crate::helpers;

impl ExchangeApi for KrakenApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_ticker_information(pair_name)?;

        let result = utils::parse_result(&raw_response)?;

        let price = helpers::from_json_bigdecimal(&result[*pair_name]["c"][0], "c")?;
        let ask = helpers::from_json_bigdecimal(&result[*pair_name]["a"][0], "a")?;
        let bid = helpers::from_json_bigdecimal(&result[*pair_name]["b"][0], "b")?;
        let vol = helpers::from_json_bigdecimal(&result[*pair_name]["v"][0], "v")?;

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
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_order_book(pair_name, "1000")?; // 1000 entries max

        let result = utils::parse_result(&raw_response)?;

        let mut ask_offers = Vec::new();
        let mut bid_offers = Vec::new();

        let ask_array =
            result[*pair_name]["asks"]
                .as_array()
                .ok_or_else(|| {
                                ErrorKind::InvalidFieldFormat(format!("{}.asks",
                                                                      result[*pair_name]))
                            })?;
        let bid_array =
            result[*pair_name]["bids"]
                .as_array()
                .ok_or_else(|| {
                                ErrorKind::InvalidFieldFormat(format!("{}.bids",
                                                                      result[*pair_name]))
                            })?;

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
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let direction = match order_type {
            OrderType::BuyLimit | OrderType::BuyMarket => "buy",
            OrderType::SellLimit | OrderType::SellMarket => "sell",
        };

        let order_type_str = match order_type {
            OrderType::BuyLimit | OrderType::SellLimit => "limit",
            OrderType::BuyMarket | OrderType::SellMarket => "market",
        };

        let mut price_str = "".to_string();
        if price.is_some() {
            price_str = price.unwrap().to_string()
        };

        let raw_response = self.add_standard_order(pair_name,
                                                   direction,
                                                   order_type_str,
                                                   &price_str,
                                                   "",
                                                   &quantity.to_string(),
                                                   "",
                                                   "",
                                                   "",
                                                   "",
                                                   "",
                                                   "")?;

        let result = utils::parse_result(&raw_response)?;

        let mut txids = Vec::new();

        let list_id =
            result["txid"]
                .as_array()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["txid"])))?;

        for id in list_id {
            txids.push(id.as_str()
                           .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", id)))?
                           .to_string());
        }

        Ok(OrderInfo {
               timestamp: helpers::get_unix_timestamp_ms(),
               identifier: txids,
           })
    }

    fn balances(&mut self) -> Result<Balances> {
        let raw_response = self.get_account_balance()?;
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
