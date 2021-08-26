//! Use this module to create a generic API.

use std::path::PathBuf;

use crate::exchange::{Exchange, ExchangeApi};
use crate::bitstamp::{BitstampApi, BitstampCreds};
use crate::kraken::{KrakenApi, KrakenCreds};
use crate::poloniex::{PoloniexApi, PoloniexCreds};
use crate::bittrex::{BittrexApi, BittrexCreds};
use crate::gdax::{GdaxApi, GdaxCreds};
use crate::error::*;

pub trait Credentials {
    /// Get an element from the credentials.
    fn get(&self, cred: &str) -> Option<String>;
    /// Return the targeted `Exchange`.
    fn exchange(&self) -> Exchange;
    /// Return the client name.
    fn name(&self) -> String;
}

#[derive(Debug)]
pub struct Coinnect;

impl Coinnect {
    /// Create a new CoinnectApi by providing an API key & API secret
    pub fn new<C: Credentials>(exchange: Exchange, creds: C) -> Result<Box<dyn ExchangeApi>> {
        match exchange {
            Exchange::Bitstamp => Ok(Box::new(BitstampApi::new(creds)?)),
            Exchange::Kraken => Ok(Box::new(KrakenApi::new(creds)?)),
            Exchange::Poloniex => Ok(Box::new(PoloniexApi::new(creds)?)),
            Exchange::Bittrex => Ok(Box::new(BittrexApi::new(creds)?)),
            Exchange::Gdax => Ok(Box::new(GdaxApi::new(creds)?)),
        }
    }

    /// Create a new CoinnectApi from a json configuration file. This file must follow this
    /// structure:
    ///
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file(Exchange::Bitstamp, "account_bitstamp", Path::new("/keys.json"))`
    pub fn new_from_file(exchange: Exchange,
                         name: &str,
                         path: PathBuf)
                         -> Result<Box<dyn ExchangeApi>> {
        match exchange {
            Exchange::Bitstamp => {
                Ok(Box::new(BitstampApi::new(BitstampCreds::new_from_file(name, path)?)?))
            }
            Exchange::Kraken => {
                Ok(Box::new(KrakenApi::new(KrakenCreds::new_from_file(name, path)?)?))
            }
            Exchange::Poloniex => {
                Ok(Box::new(PoloniexApi::new(PoloniexCreds::new_from_file(name, path)?)?))
            }
            Exchange::Bittrex => {
                Ok(Box::new(BittrexApi::new(BittrexCreds::new_from_file(name, path)?)?))
            }
            Exchange::Gdax => {
                Ok(Box::new(GdaxApi::new(GdaxCreds::new_from_file(name, path)?)?))
            }
        }
    }
}
