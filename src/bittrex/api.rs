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

    /// Used to get all supported currencies at Bittrex along with other meta data.
    /// 
    /// ```json
    /// {
    ///     "success" : true,
    ///     "message" : "",
    ///     "result" : [{
    ///             "Currency" : "BTC",
    ///             "CurrencyLong" : "Bitcoin",
    ///             "MinConfirmation" : 2,
    ///             "TxFee" : 0.00020000,
    ///             "IsActive" : true,
    ///             "CoinType" : "BITCOIN",
    ///             "BaseAddress" : null
    ///         }, {
    ///             "Currency" : "LTC",
    ///             "CurrencyLong" : "Litecoin",
    ///             "MinConfirmation" : 5,
    ///             "TxFee" : 0.00200000,
    ///             "IsActive" : true,
    ///             "CoinType" : "BITCOIN",
    ///             "BaseAddress" : null
    ///         }
    ///     ]
    /// }
    /// ```
    pub fn get_currencies(&mut self) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        self.public_query("/public/getcurrencies", &mut params)
    }

    /// Used to get the current tick values for a market.
    /// "market" required a string literal for the market (ex: BTC-LTC)
    /// 
    /// ````json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 		"Bid" : 2.05670368,
    /// 		"Ask" : 3.35579531,
    /// 		"Last" : 3.35579531
    /// 	}
    /// }
    /// ```
    pub fn get_ticker(&mut self, market: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        self.public_query("/public/getticker", &mut params)
    }

    /// Used to get the last 24 hour summary of all active exchanges
    /// 
    /// ````json
    /// {
    ///     "success" : true,
    ///     "message" : "",
    ///     "result" : [{
    ///             "MarketName" : "BTC-888",
    ///             "High" : 0.00000919,
    ///             "Low" : 0.00000820,
    ///             "Volume" : 74339.61396015,
    ///             "Last" : 0.00000820,
    ///             "BaseVolume" : 0.64966963,
    ///             "TimeStamp" : "2014-07-09T07:19:30.15",
    ///             "Bid" : 0.00000820,
    ///             "Ask" : 0.00000831,
    ///             "OpenBuyOrders" : 15,
    ///             "OpenSellOrders" : 15,
    ///             "PrevDay" : 0.00000821,
    ///             "Created" : "2014-03-20T06:00:00",
    ///             "DisplayMarketName" : null
    ///         }, {
    ///             "MarketName" : "BTC-A3C",
    ///             "High" : 0.00000072,
    ///             "Low" : 0.00000001,
    ///             "Volume" : 166340678.42280999,
    ///             "Last" : 0.00000005,
    ///             "BaseVolume" : 17.59720424,
    ///             "TimeStamp" : "2014-07-09T07:21:40.51",
    ///             "Bid" : 0.00000004,
    ///             "Ask" : 0.00000005,
    ///             "OpenBuyOrders" : 18,
    ///             "OpenSellOrders" : 18,
    ///             "PrevDay" : 0.00000002,
    ///             "Created" : "2014-05-30T07:57:49.637",
    ///             "DisplayMarketName" : null
    ///         }
    ///     ]
    /// }
    /// ```
    pub fn get_market_summaries(&mut self) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        self.public_query("/public/getmarketsummaries", &mut params)
    }

    /// Used to get the last 24 hour summary of all active exchanges
    /// "market" required a string literal for the market (ex: BTC-LTC)
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"MarketName" : "BTC-LTC",
    /// 			"High" : 0.01350000,
    /// 			"Low" : 0.01200000,
    /// 			"Volume" : 3833.97619253,
    /// 			"Last" : 0.01349998,
    /// 			"BaseVolume" : 47.03987026,
    /// 			"TimeStamp" : "2014-07-09T07:22:16.72",
    /// 			"Bid" : 0.01271001,
    /// 			"Ask" : 0.01291100,
    /// 			"OpenBuyOrders" : 45,
    /// 			"OpenSellOrders" : 45,
    /// 			"PrevDay" : 0.01229501,
    /// 			"Created" : "2014-02-13T00:00:00",
    /// 			"DisplayMarketName" : null
    /// 		}
    ///     ]
    /// }
    /// ```
    pub fn get_market_summary(&mut self, market: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        self.public_query("/public/getmarketsummary", &mut params)
    }

    /// Used to get retrieve the orderbook for a given market
    /// "market" required a string literal for the market (ex: BTC-LTC)
    /// "order_type" required "buy", "sell" or "both" to identify the type of orderbook to return.
    /// 
    /// ```json
    /// {
    ///     "success" : true,
    ///     "message" : "",
    ///     "result" : {
    ///         "buy" : [{
    ///                 "Quantity" : 12.37000000,
    ///                 "Rate" : 0.02525000
    ///             }
    ///         ],
    ///         "sell" : [{
    ///                 "Quantity" : 32.55412402,
    ///                 "Rate" : 0.02540000
    ///             }, {
    ///                 "Quantity" : 60.00000000,
    ///                 "Rate" : 0.02550000
    ///             }, {
    ///                 "Quantity" : 60.00000000,
    ///                 "Rate" : 0.02575000
    ///             }, {
    ///                 "Quantity" : 84.00000000,
    ///                 "Rate" : 0.02600000
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    pub fn get_order_book(&mut self, market: &str, order_type: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        params.insert("type", order_type);
        self.public_query("/public/getorderbook", &mut params)
    }

    /// Used to retrieve the latest trades that have occured for a specific market.
    /// "market" required a string literal for the market (ex: BTC-LTC)
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"Id" : 319435,
    /// 			"TimeStamp" : "2014-07-09T03:21:20.08",
    /// 			"Quantity" : 0.30802438,
    /// 			"Price" : 0.01263400,
    /// 			"Total" : 0.00389158,
    /// 			"FillType" : "FILL",
    /// 			"OrderType" : "BUY"
    /// 		}, {
    /// 			"Id" : 319433,
    /// 			"TimeStamp" : "2014-07-09T03:21:20.08",
    /// 			"Quantity" : 0.31820814,
    /// 			"Price" : 0.01262800,
    /// 			"Total" : 0.00401833,
    /// 			"FillType" : "PARTIAL_FILL",
    /// 			"OrderType" : "BUY"
    /// 		}, {
    /// 			"Id" : 319379,
    /// 			"TimeStamp" : "2014-07-09T02:58:48.127",
    /// 			"Quantity" : 49.64643541,
    /// 			"Price" : 0.01263200,
    /// 			"Total" : 0.62713377,
    /// 			"FillType" : "FILL",
    /// 			"OrderType" : "SELL"
    /// 		}, {
    /// 			"Id" : 319378,
    /// 			"TimeStamp" : "2014-07-09T02:58:46.27",
    /// 			"Quantity" : 0.35356459,
    /// 			"Price" : 0.01263200,
    /// 			"Total" : 0.00446622,
    /// 			"FillType" : "PARTIAL_FILL",
    /// 			"OrderType" : "BUY"
    /// 		}
    /// 	]
    /// }
    /// ```
    pub fn get_market_history(&mut self, market: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        self.public_query("/public/getmarkethistory", &mut params)
    }
}
