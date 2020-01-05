//! Use this module to interact with Bittrex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bittrex offers.

use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::exchange::{ExchangeApi, FResult};
use crate::bittrex::api::BittrexApi;

use crate::error::*;
use crate::types::*;
use crate::bittrex::utils;
use crate::helpers;
use async_trait::async_trait;

use futures::{Future, Stream};

#[async_trait]
impl ExchangeApi for BittrexApi {
    async fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_market_summary(pair_name).await?;

        let result = utils::parse_result(&raw_response)?;
        let result_array = result.as_array();
        let result_obj = result_array.unwrap()[0].as_object().unwrap();

        let price_str = result_obj.get("Last").unwrap().as_f64().unwrap().to_string();
        let price = BigDecimal::from_str(&price_str).unwrap();

        let ask_str = result_obj.get("Ask").unwrap().as_f64().unwrap().to_string();
        let ask = BigDecimal::from_str(&ask_str).unwrap();

        let bid_str = result_obj.get("Bid").unwrap().as_f64().unwrap().to_string();
        let bid = BigDecimal::from_str(&bid_str).unwrap();

        let volume_str = result_obj.get("Volume").unwrap().as_f64().unwrap().to_string();
        let vol = BigDecimal::from_str(&volume_str).unwrap();

        Ok(Ticker {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            last_trade_price: price,
            lowest_ask: ask,
            highest_bid: bid,
            volume: Some(vol),
        })

    }

    async fn orderbook(&mut self, pair: Pair) -> Result<Orderbook> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_order_book(pair_name, "both").await?;

        let result = utils::parse_result(&raw_response)?;

        let mut ask_offers = Vec::new();    // buy orders
        let mut bid_offers = Vec::new();    // sell orders

        let buy_orders = result["buy"].as_array()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["buy"])))?;

        let sell_orders = result["sell"].as_array()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["sell"])))?;

        for ask in buy_orders {
            let ask_obj = ask.as_object().unwrap();

            let price_str = ask_obj.get("Rate").unwrap().as_f64().unwrap().to_string();
            let price = BigDecimal::from_str(&price_str).unwrap();


            let volume_str = ask_obj.get("Quantity").unwrap().as_f64().unwrap().to_string();
            let volume = BigDecimal::from_str(&volume_str).unwrap();

            ask_offers.push((price, volume));
        }

        for bid in sell_orders {
            let bid_obj = bid.as_object().unwrap();

            let price_str = bid_obj.get("Rate").unwrap().as_f64().unwrap().to_string();
            let price = BigDecimal::from_str(&price_str).unwrap();


            let volume_str = bid_obj.get("Quantity").unwrap().as_f64().unwrap().to_string();
            let volume = BigDecimal::from_str(&volume_str).unwrap();

            bid_offers.push((price, volume));
        }

        Ok(Orderbook {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            asks: ask_offers,
            bids: bid_offers,
        })
    }

    async fn add_order(&mut self,
                 order_type: OrderType,
                 pair: Pair,
                 quantity: Volume,
                 price: Option<Price>)
                 -> Result<OrderInfo> {

        let pair_name = match utils::get_pair_string(&pair) {
            Some(pair_str) => pair_str,
            None => return Err(ErrorKind::PairUnsupported.into())
        };

        let raw_response = match order_type {
            OrderType::BuyLimit => {
                if price.is_none() {
                    return Err(ErrorKind::MissingPrice.into());
                }
                self.buy_limit(pair_name, &quantity.to_string(), &price.unwrap().to_string()).await
            }
            OrderType::BuyMarket => {
                let min_price = "0.000000001";
                self.buy_limit(pair_name, &quantity.to_string(), min_price).await
            }
            OrderType::SellLimit => {
                if price.is_none() {
                    return Err(ErrorKind::MissingPrice.into());
                }
                self.sell_limit(pair_name, &quantity.to_string(), &price.unwrap().to_string()).await
            }
            OrderType::SellMarket => {
                let max_price = "999999999.99";
                self.buy_limit(pair_name, &quantity.to_string(), max_price).await
            }
        }?;

        let result = utils::parse_result(&raw_response)?;

        let result_obj = result.as_object().unwrap();

        Ok(OrderInfo {
               timestamp: helpers::get_unix_timestamp_ms(),
               identifier: vec![result_obj.get("uuid").unwrap().as_str().unwrap().to_string()],
        })
    }

    async fn balances(&mut self) -> Result<Balances> {
        let raw_response = self.get_balances().await?;

        let result = utils::parse_result(&raw_response)?;

        let result_array = result.as_array().unwrap();

        let mut balances = Balances::new();

        for currency in result_array {
            let currency_obj = currency.as_object().unwrap();
            let currency_str = currency_obj.get("Currency").unwrap().as_str().unwrap();
            let currency = utils::get_currency_enum(&currency_str);

            match currency {
                Some(c) => {
                    let amount_str = currency_obj.get("Available").unwrap().as_f64().unwrap().to_string();
                    let amount = BigDecimal::from_str(&amount_str).unwrap();
                    balances.insert(c, amount);
                },
                _ => ()
            }
        }
        Ok(balances)
    }
}
