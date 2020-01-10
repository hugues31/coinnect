//! Use this module to interact with Bittrex exchange.
//! See examples for more informations.

pub mod api;
pub mod generic_api;
pub mod credentials;
pub mod utils;
pub mod streaming_api;
pub mod models;

pub use self::credentials::BittrexCreds;
pub use self::api::BittrexApi;
