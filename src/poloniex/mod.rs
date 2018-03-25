//! Use this module to interact with Poloniex exchange.

pub mod api;
pub mod generic_api;
pub mod credentials;
pub mod utils;

pub use self::credentials::PoloniexCreds;
pub use self::api::PoloniexApi;
pub use self::api::{MoveOrderOption, PlaceOrderOption};
