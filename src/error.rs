//! This module contains enum Error.
//! Error type represents all possible errors that can occur when dealing
//! with the generic or any dedicated-exchange API


use serde_json;
use hyper;
use data_encoding;
use exchange::Exchange;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Json(serde_json::Error);
        ParseFloat(::std::num::ParseFloatError);
        ParseString(::std::string::FromUtf8Error);
        Hyper(hyper::Error);
        DataDecoding(data_encoding::DecodeError);
        Io(::std::io::Error);
    }

    errors {
        BadParse {
            description("ParsingError")
                display("The response could not be parsed.")
        }

        ServiceUnavailable(reason: String) {
            description("ServiceUnavailable")
                display("Host could not be reached: {}.", reason)
        }

        BadCredentials {
            description("BadCredentials")
                display("The informations provided do not allow authentication.")
        }

        RateLimitExceeded {
            description("RateLimitExceeded")
                display("API call rate limit exceeded.")
        }

        PairUnsupported {
            description("PairUnsupported")
                display("This pair is not supported.")
        }

        InvalidArguments {
            description("InvalidArguments")
                display("Arguments passed do not conform to the protocol.")
        }

        ExchangeSpecificError(reason: String) {
            description("ExchangeSpecificError")
                display("Exchange error: {}", reason)
        }

        TlsError {
            description("TlsError")
                display("Fail to initialize TLS client.")
        }

        InvalidFieldFormat(field: String) {
            description("InvalidFieldFormat")
                display("Fail to parse field \"{}\".", field)
        }

        InvalidFieldValue(field: String) {
            description("InvalidFieldValue")
                display("Invalid value for field \"{}\".", field)
        }

        MissingField(field: String) {
            description("MissingFiled")
                display("Missing field \"{}\".", field)
        }

        InsufficientFunds {
            description("InsufficientFunds")
                display("You haven't enough founds.")
        }

        InsufficientOrderSize {
            description("InsufficientOrderSize")
                display("Your order is not big enough.")
        }

        MissingPrice{
            description("MissingPrice")
                display("No price specified.")
        }

        InvalidConfigType(expected: Exchange, find: Exchange){
            description("InvalidConfigType")
                display("Invalid config: \nExpected: {:?}\nFind: {:?}", expected, find)
        }

        InvalidExchange(value: String) {
            description("InvalidExchange")
                display("Invalid exchange: \"{}\"", value)
        }
    }
}
