//! ![Coinnect](https://raw.githubusercontent.com/hugues31/coinnect/master/coinnect.png)
//!
//! Coinnect is a Rust library aiming to provide a complete access to REST APIs for various
//! crypto-currencies exchanges. All methods consume HTTPS api. TThe purpose of this crate is not
//! to stream data (you should use websocket/FIX in that case).
//!
//! Currently only Poloniex and Kraken are supported but other exchanges will be added soon.
//!
//! For optional parameters, enter an empty &str ("") if you don't specify it.
//!
//! ### Exchange support:
//! - [x] Poloniex
//! - [x] Kraken
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

pub mod poloniex;
pub mod kraken;
mod helpers;

#[cfg(test)]
mod tests {}
