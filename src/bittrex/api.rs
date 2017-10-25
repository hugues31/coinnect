//! Use this module to interact with the raw-original API provided by Bittrex.
//! WARNING: Special attention should be paid to error management: parsing number, etc.

#![allow(too_many_arguments)]

use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512, Digest};

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;

use data_encoding::BASE64;

use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::str;

use error::*;
use helpers;

use exchange::Exchange;
use coinnect::Credentials;
use bittrex::utils;

header! {
    #[doc(hidden)]
    (ApiSign, "apisign") => [String]
}

#[derive(Debug)]
pub struct BittrexApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
    burst: bool,
}


impl BittrexApi {
    /// Create a new BittrexApi by providing an API key & API secret
    pub fn new<C: Credentials>(creds: C) -> Result<BittrexApi> {
        if creds.exchange() != Exchange::Bittrex {
            return Err(ErrorKind::InvalidConfigType(Exchange::Bittrex, creds.exchange()).into());
        }

        // TODO: implement correctly the TLS error in error_chain.
        let ssl = match NativeTlsClient::new() {
            Ok(res) => res,
            Err(_) => return Err(ErrorKind::TlsError.into()),
        };
        let connector = HttpsConnector::new(ssl);

        Ok(BittrexApi {
               last_request: 0,
               api_key: creds.get("api_key").unwrap_or_default(),
               api_secret: creds.get("api_secret").unwrap_or_default(),
               http_client: Client::with_connector(connector),
               burst: false,
           })
    }

    /// The number of calls in a given period is limited. In order to avoid a ban we limit
    /// by default the number of api requests.
    /// This function sets or removes the limitation.
    /// Burst false implies no block.
    /// Burst true implies there is a control over the number of calls allowed to the exchange
    pub fn set_burst(&mut self, burst: bool) {
        self.burst = burst
    }

    pub fn block_or_continue(&self) {
        if ! self.burst {
            let threshold: u64 = 500; // 1 request/500ms
            let offset: u64 = helpers::get_unix_timestamp_ms() as u64 - self.last_request as u64;
            if offset < threshold {
                let wait_ms = Duration::from_millis(threshold - offset);
                thread::sleep(wait_ms);
            }
        }
    }

    fn public_query(&mut self,
                    method: &str,
                    params: &mut HashMap<&str, &str>)
                    -> Result<Map<String, Value>> {

        helpers::strip_empties(params);

        let url = "https://bittrex.com/api/v1.1".to_string() + method + "?" +
                  &helpers::url_encode_hashmap(params);

        self.block_or_continue();
        //TODO: Handle correctly http errors with error_chain.
        let mut response = match self.http_client.get(&url).send() {
            Ok(response) => response,
            Err(err) => return Err(ErrorKind::ServiceUnavailable(err.to_string()).into()),
        };
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        utils::deserialize_json(&buffer)
    }

    fn private_query(&mut self,
                     method: &str,
                     mut params: &mut HashMap<&str, &str>)
                     -> Result<Map<String, Value>> {
        let url = "https://bittrex.com/api/v1.1".to_string() + method;

        unimplemented!();
    }

    fn create_signature(&self, urlpath: String, postdata: &str, nonce: &str) -> Result<String> {
        unimplemented!();
    }

    /// Used to get the open and available trading markets at Bittrex along with other meta data.
    ///
    /// ```json
    ///    {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"MarketCurrency" : "LTC",
    /// 			"BaseCurrency" : "BTC",
    /// 			"MarketCurrencyLong" : "Litecoin",
    /// 			"BaseCurrencyLong" : "Bitcoin",
    /// 			"MinTradeSize" : 0.01000000,
    /// 			"MarketName" : "BTC-LTC",
    /// 			"IsActive" : true,
    /// 			"Created" : "2014-02-13T00:00:00"
    /// 		}, {
    /// 			"MarketCurrency" : "DOGE",
    /// 			"BaseCurrency" : "BTC",
    /// 			"MarketCurrencyLong" : "Dogecoin",
    /// 			"BaseCurrencyLong" : "Bitcoin",
    /// 			"MinTradeSize" : 100.00000000,
    /// 			"MarketName" : "BTC-DOGE",
    /// 			"IsActive" : true,
    /// 			"Created" : "2014-02-13T00:00:00"
    /// 		}
    ///     ]
    /// }
    /// ```
    pub fn get_markets(&mut self) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        self.public_query("/public/getmarkets", &mut params)
    }
}
