//! Use this module to interact with Bitstamp exchange.

pub mod api;
pub mod generic_api;
pub mod credentials;
pub mod utils;
pub mod models;
pub mod streaming_api;
pub use self::credentials::BitstampCreds;
pub use self::api::BitstampApi;
