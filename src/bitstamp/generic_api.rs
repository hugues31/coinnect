//! Use this module to interact with Bitstamp through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bitstamp offers.

use exchange::ExchangeApi;
use bitstamp::api::BitstampApi;

use error::*;
use pair::Pair;
//use bitstamp::utils;
use types::*;
use helpers;

impl ExchangeApi for BitstampApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {

        let result = self.return_ticker(pair)?;

        //let parse_as_float = |field: &str| field.parse::<f64>()?;

        let price = result["last"]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField("last".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("last".to_string()))?;

        let ask = result["ask"]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField("ask".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("ask".to_string()))?;
        let bid = result["bid"]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField("bid".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("bid".to_string()))?;
        let vol = result["volume"]
            .as_str()
            .ok_or_else(|| ErrorKind::MissingField("volume".to_string()))?
            .parse::<f64>()
            .chain_err(|| ErrorKind::InvalidFieldFormat("volume".to_string()))?;

        Ok(Ticker {
               timestamp: helpers::get_unix_timestamp_ms(),
               pair: pair,
               last_trade_price: price,
               lowest_ask: ask,
               highest_bid: bid,
               volume: Some(vol),
           })
    }

    fn orderbook(&mut self,
                 /*pair*/
                 _: Pair)
                 -> Result<Orderbook> {
        unimplemented!();
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
}
