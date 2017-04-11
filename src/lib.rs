//! ![Coinnect](https://raw.githubusercontent.com/hugues31/coinnect/master/coinnect.png)
//!
//! Coinnect is a Rust library aiming to provide a complete access to REST APIs for various
//! crypto-currencies exchanges (see below for a list of supported exchanges).
//! All methods consume HTTPS api. The purpose of this crate is not
//! to stream data (you should use websocket/FIX in that case).
//!
//! For optional parameters, most methods require an empty str (`""`) if you don't want to specify
//! them.
//!
//! ### Exchanges support:
//! - [x] Poloniex
//! - [x] Kraken
//! - [x] Bitstamp (partial)
//!
//! # WARNING
//! This library is highly experimental at the moment. Please do not invest what you
//! can't afford to loose. This is a personal project, I can not be held responsible for
//! the library malfunction, which can lead to a loss of money.

#[macro_use]
extern crate hyper;
extern crate crypto;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate serde_json;
extern crate time;
#[macro_use]
extern crate lazy_static;
extern crate bidir_map;

pub mod coinnect;
pub mod exchange;
pub mod error;
pub mod pair;
pub mod types;
mod helpers;

pub mod bitstamp;
pub mod poloniex;
pub mod kraken;
