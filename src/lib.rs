//! ![Coinnect](../../../coinnect.png)
//!
//! Coinnect is a Rust library aiming to provide a complete access to REST APIs for various
//! crypto-currencies exchanges. All methods consume HTTPS api. This library is not intendeed to
//! stream data (you should use websocket/FIX in that case)..
//!
//! Currently only Poloniex is supported but other exchanges will be added soon.
//!
//! ### Exchange support:
//! - [x] Poloniex (90%)
//! - [x] Kraken (20%)
//!
//! # WARNING
//! This library is highly experimental at the moment. Please do not invest what you
//! can't afford to loose.

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
mod tests {

}
