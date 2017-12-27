//! Use this module to interact with Bitstamp exchange.

pub mod api;
pub mod generic_api;
pub mod credentials;
pub mod utils;

pub use self::credentials::GdaxCreds;
pub use self::api::GdaxApi;
