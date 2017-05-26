//! Use this module to interact with Kraken through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Kraken offers.

use exchange::ExchangeApi;
use kraken::api::KrakenApi;

use error::*;
use pair::Pair;
use types::*;
use kraken::utils;
use helpers;

impl ExchangeApi for KrakenApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_ticker_information(pair_name)?;

        let result = utils::parse_result(&raw_response)?;

        let price = result[*pair_name]["c"][0]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField(format!("{}.c", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.c", pair_name)))?;
        let ask = result[*pair_name]["a"][0]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField(format!("{}.a", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.a", pair_name)))?;
        let bid = result[*pair_name]["b"][0]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField(format!("{}.b", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.b", pair_name)))?;
        let vol = result[*pair_name]["v"][1]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField(format!("{}.v", pair_name)))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}.v", pair_name)))?;

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
            let price = ask[0]
                .as_str()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", ask[0])))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}", ask[0])))?;
            let volume = ask[1]
                .as_str()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", ask[1])))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}", ask[1])))?;
            ask_offers.push((price, volume));
        }

        for bid in bid_array {
            let price = bid[0]
                .as_str()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", bid[0])))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}", bid[0])))?;
            let volume = bid[1]
                .as_str()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", bid[1])))?
                .parse::<f64>()
                .chain_err(|| ErrorKind::InvalidFieldFormat(format!("{}", bid[1])))?;

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
        unimplemented!();
    }
}
