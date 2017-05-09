//! This module contains enum Error.
//! Error type represents all possible errors that can occur when dealing
//! with the generic or any dedicated-exchange API

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ServiceUnavailable,
    BadParse,
    InvalidLogin,
    InvalidArguments,
    RateLimitExceeded,
    PairUnsupported,
    ExchangeSpecificError(String),
    UndefinedError,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ServiceUnavailable => "Host could not be reached.",
            Error::BadParse => "The response could not be parsed.",
            Error::InvalidLogin => "Wrong API key or secret.",
            Error::InvalidArguments => "Arguments passed do not conform to the protocol.",
            Error::RateLimitExceeded => "API call rate limit exceeded.",
            Error::PairUnsupported => "This pair is not supported.",
            Error::ExchangeSpecificError(ref s) => s,
            Error::UndefinedError => "An unknown error occurred.",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => error::Error::description(self).fmt(f),
        }
    }
}
