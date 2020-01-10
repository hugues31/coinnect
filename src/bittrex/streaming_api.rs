use crate::coinnect::Credentials;
use crate::exchange_bot::{DefaultWsActor, WsHandler, ExchangeBot};
use crate::error::*;
use super::models::*;
use bytes::Bytes;
use bytes::Buf;
use serde_json::Value;
use std::io::Read;
use futures::stream::{SplitSink, StreamExt};
use actix::{Context, io::SinkWrite, Actor, Handler, StreamHandler, AsyncContext, ActorContext, Addr, SystemService, Recipient};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    Client, BoxedSocket
};
use actix_codec::{AsyncRead, AsyncWrite, Framed};
use actix_rt::{System, Arbiter};
use crate::types::{LiveEvent, Channel, Orderbook, Pair, LiveAggregatedOrderBook};
use signalr_rs::hub::client::{HubClientError, HubClientHandler, HubClient, HubQuery};
use serde::de::DeserializeOwned;
use libflate::deflate::Decoder;
use chrono::prelude::*;
use bigdecimal::BigDecimal;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct BittrexStreamingApi {
    api_key: String,
    api_secret: String,
    customer_id: String,
    currency_pair: String,
    pub recipients: Vec<Recipient<LiveEvent>>,
    channels: Vec<Channel>,
    agg: LiveAggregatedOrderBook
}

pub struct BittrexBot {
    addr: Addr<HubClient>
}

impl ExchangeBot for BittrexBot {
    fn is_connected(&self) -> bool {
        unimplemented!()
    }
}

impl BittrexStreamingApi {
    pub async fn new_bot<C: Credentials>(creds: C, currency_pair: String, channels: Vec<Channel>, recipients: Vec<Recipient<LiveEvent>>) -> Result<BittrexBot> {
        let hub = "c2";
        let api = Box::new(BittrexStreamingApi {
            api_key: creds.get("api_key").unwrap_or_default(),
            api_secret: creds.get("api_secret").unwrap_or_default(),
            customer_id: creds.get("customer_id").unwrap_or_default(),
            currency_pair,
            recipients,
            channels,
            agg: LiveAggregatedOrderBook {
                depth: 5,
                pair: Pair::BTC_USD,
                asks_by_price: IndexMap::new(),
                bids_by_price: IndexMap::new()
            },
        });
        let client = HubClient::new(hub, "https://socket.bittrex.com/signalr/", api).await;
        match client {
            Ok(addr) => {
//                addr.do_send(HubQuery::new(hub.to_string(), "SubscribeToSummaryDeltas".to_string(), "".to_string(), "0".to_string()));
                addr.do_send(HubQuery::new(hub.to_string(), "SubscribeToExchangeDeltas".to_string(), vec!["USDT-BTC"], "1".to_string()));
//                addr.do_send(HubQuery::new(hub.to_string(), "QueryExchangeState".to_string(), vec!["USDT-BTC"], "QE2".to_string()));
                return Ok(BittrexBot { addr });
            }
            Err(e) => {
                return Err(ErrorKind::Hub(e).into());
            }
        }
    }
}

impl BittrexStreamingApi {
    fn deflate<T>(binary: &String) -> Result<T> where T: DeserializeOwned {
        let decoded = base64::decode(binary).map_err(|e| ErrorKind::Hub(HubClientError::Base64DecodeError(e)))?;
        let mut decoder = Decoder::new(&decoded[..]);
        let mut decoded_data : Vec<u8> = Vec::new();
        decoder.read_to_end(&mut decoded_data);
        let v: &[u8] = &decoded_data;
        println!("{:?}", std::str::from_utf8(v));
        serde_json::from_slice::<T>(v).map_err(|e| ErrorKind::Hub(HubClientError::ParseError(e)).into())
    }

    fn deflate_array<T>(a: &Value) -> Result<T> where T: DeserializeOwned {
        let data: Vec<String> = serde_json::from_value(a.clone())?;
        let binary = data.first().ok_or(ErrorKind::Hub(HubClientError::MissingData))?;
        BittrexStreamingApi::deflate::<T>(binary)
    }

    fn deflate_string<T>(a: &Value) -> Result<T> where T: DeserializeOwned {
        let binary: String = serde_json::from_value(a.clone())?;
        BittrexStreamingApi::deflate::<T>(&binary)
    }
}

impl HubClientHandler for BittrexStreamingApi {
    fn connected(&self) {}

    fn error(&self, id: Option<&str>, msg: &Value) {}

        fn handle(&mut self, method: &str, message: &Value) {
        let live_event = match method {
            "uE" => {
                BittrexStreamingApi::deflate_array::<MarketDelta>(message);
                Err(())
            },
            "uS" => {
                BittrexStreamingApi::deflate_array::<SummaryDeltaResponse>(message);
                Err(())
            },
            s if s.starts_with("QE") => {
                let state = BittrexStreamingApi::deflate_string::<ExchangeState>(message).unwrap();
                state.Sells.into_iter().map(|op| {
                    let kp = (BigDecimal::from(op.R), BigDecimal::from(op.Q));
                    self.agg.asks_by_price.entry(kp.0.clone()).or_insert(kp);
                });
                state.Buys.into_iter().map(|op| {
                    let kp = (BigDecimal::from(op.R), BigDecimal::from(op.Q));
                    self.agg.bids_by_price.entry(kp.0.clone()).or_insert(kp);
                });
                println!("{:?}", self.agg);
                let latest_order_book : Orderbook = self.agg.order_book(10);
                Ok(LiveEvent::LiveOrderbook(latest_order_book.clone()))
            },
            _ => {
                debug!("Unknown message : method {:?} message {:?}", method, message);
                Err(())
            }
        };
        if live_event.is_ok() {
            let le = live_event.unwrap();
            let vec = self.recipients.clone();
            for r in &vec {
                let le : LiveEvent = le.clone();
                r.do_send(le);
            }

        }
    }
}
