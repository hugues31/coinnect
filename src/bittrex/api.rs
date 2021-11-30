//! Use this module to interact with the raw-original API provided by Bittrex.
//! WARNING: Special attention should be paid to error management: parsing number, etc.

use hmac::{Hmac, Mac, NewMac};
use sha2::{Sha512};

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;

use data_encoding::HEXLOWER;

use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::str;

use crate::error::*;
use crate::helpers;

use crate::exchange::Exchange;
use crate::coinnect::Credentials;
use crate::bittrex::utils;

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
        let nonce = helpers::get_unix_timestamp_ms().to_string();
        let mut initial_params: HashMap<&str, &str> = HashMap::new();
        
        initial_params.insert("nonce", &nonce);
        initial_params.insert("apikey", &self.api_key);

        let base_url = "https://bittrex.com/api/v1.1".to_string() + method + "?apikey=" +
        &self.api_key + "&nonce=" + &nonce;
        
        let url = if params.is_empty() {
            base_url
        } else {
            base_url + "&" + &helpers::url_encode_hashmap(&mut params)
        };
 
        let hmac_key = self.api_secret.as_bytes();
        let mut mac = Hmac::<Sha512>::new_from_slice(&hmac_key[..]).unwrap();
        mac.update(url.as_bytes());

        let mut custom_header = header::Headers::new();

        let signature = HEXLOWER.encode(&mac.finalize().into_bytes());

        custom_header.set(ApiSign(signature));

        let mut res = match self.http_client
                  .post(&url)
                  .headers(custom_header)
                  .send() {
            Ok(res) => res,
            Err(err) => return Err(ErrorKind::ServiceUnavailable(err.to_string()).into()),
        };

        let mut buffer = String::new();
        res.read_to_string(&mut buffer)?;
        utils::deserialize_json(&buffer)
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

    /// Used to place a buy order in a specific market. Use buylimit to place limit orders.
    /// Make sure you have the proper permissions set on your API keys for this call to work.
    /// "market" required a string literal for the market (ex: BTC-LTC)
    /// "quantity" required the amount to purchase
    /// "rate" required the rate at which to place the order.
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 			"uuid" : "e606d53c-8d70-11e3-94b5-425861b86ab6"
    /// 	}
    /// }
    /// ```
    pub fn buy_limit(&mut self, market: &str, quantity: &str, rate: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        params.insert("quantity", quantity);
        params.insert("rate", rate);
        self.private_query("/market/buylimit", &mut params)
    }

    /// Used to place a sell order in a specific market. Use selllimit to place limit orders.
    /// Make sure you have the proper permissions set on your API keys for this call to work.
    /// "market" required a string literal for the market (ex: BTC-LTC)
    /// "quantity" required the amount to purchase
    /// "rate" required the rate at which to place the order.
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 			"uuid" : "614c34e4-8d71-11e3-94b5-425861b86ab6"
    /// 	}
    /// }
    /// ```
    pub fn sell_limit(&mut self, market: &str, quantity: &str, rate: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        params.insert("quantity", quantity);
        params.insert("rate", rate);
        self.private_query("/market/selllimit", &mut params)
    }

    /// Used to cancel a buy or sell order.
    /// "uuid" required uuid of buy or sell order
    /// 
    /// ```json
    /// {
    /// "success" : true,
    /// "message" : "",
    /// "result" : null
    /// }
    /// ```
    pub fn cancel(&mut self, uuid: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("uuid", uuid);
        self.private_query("/market/cancel", &mut params)
    }

    /// Get all orders that you currently have opened. A specific market can be requested
    /// "market" optional a string literal for the market (ie. BTC-LTC)
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"Uuid" : null,
    /// 			"OrderUuid" : "09aa5bb6-8232-41aa-9b78-a5a1093e0211",
    /// 			"Exchange" : "BTC-LTC",
    /// 			"OrderType" : "LIMIT_SELL",
    /// 			"Quantity" : 5.00000000,
    /// 			"QuantityRemaining" : 5.00000000,
    /// 			"Limit" : 2.00000000,
    /// 			"CommissionPaid" : 0.00000000,
    /// 			"Price" : 0.00000000,
    /// 			"PricePerUnit" : null,
    /// 			"Opened" : "2014-07-09T03:55:48.77",
    /// 			"Closed" : null,
    /// 			"CancelInitiated" : false,
    /// 			"ImmediateOrCancel" : false,
    /// 			"IsConditional" : false,
    /// 			"Condition" : null,
    /// 			"ConditionTarget" : null
    /// 		}, {
    /// 			"Uuid" : null,
    /// 			"OrderUuid" : "8925d746-bc9f-4684-b1aa-e507467aaa99",
    /// 			"Exchange" : "BTC-LTC",
    /// 			"OrderType" : "LIMIT_BUY",
    /// 			"Quantity" : 100000.00000000,
    /// 			"QuantityRemaining" : 100000.00000000,
    /// 			"Limit" : 0.00000001,
    /// 			"CommissionPaid" : 0.00000000,
    /// 			"Price" : 0.00000000,
    /// 			"PricePerUnit" : null,
    /// 			"Opened" : "2014-07-09T03:55:48.583",
    /// 			"Closed" : null,
    /// 			"CancelInitiated" : false,
    /// 			"ImmediateOrCancel" : false,
    /// 			"IsConditional" : false,
    /// 			"Condition" : null,
    /// 			"ConditionTarget" : null
    /// 		}
    /// 	]
    /// }
    /// ```
    pub fn get_open_orders(&mut self, market: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        self.private_query("/market/getopenorders", &mut params)
    }

    /// Used to retrieve all balances from your account
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"Currency" : "DOGE",
    /// 			"Balance" : 0.00000000,
    /// 			"Available" : 0.00000000,
    /// 			"Pending" : 0.00000000,
    /// 			"CryptoAddress" : "DLxcEt3AatMyr2NTatzjsfHNoB9NT62HiF",
    /// 			"Requested" : false,
    /// 			"Uuid" : null
    /// 
    /// 		}, {
    /// 			"Currency" : "BTC",
    /// 			"Balance" : 14.21549076,
    /// 			"Available" : 14.21549076,
    /// 			"Pending" : 0.00000000,
    /// 			"CryptoAddress" : "1Mrcdr6715hjda34pdXuLqXcju6qgwHA31",
    /// 			"Requested" : false,
    /// 			"Uuid" : null
    /// 		}
    /// 	]
    /// }
    /// ```
    pub fn get_balances(&mut self) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        self.private_query("/account/getbalances", &mut params)
    }

    /// Used to retrieve the balance from your account for a specific currency.
    /// "currency" required a string literal for the currency (ex: LTC)
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 		"Currency" : "BTC",
    /// 		"Balance" : 4.21549076,
    /// 		"Available" : 4.21549076,
    /// 		"Pending" : 0.00000000,
    /// 		"CryptoAddress" : "1MacMr6715hjds342dXuLqXcju6fgwHA31",
    /// 		"Requested" : false,
    /// 		"Uuid" : null
    /// 	}
    /// }
    /// ```
    pub fn get_balance(&mut self, currency: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.private_query("/account/getbalance", &mut params)
    }

    /// Used to retrieve or generate an address for a specific currency.
    /// If one does not exist, the call will fail and return ADDRESS_GENERATING until one is available.
    /// "currency" required a string literal for the currency (ex: LTC)
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 		"Currency" : "VTC",
    /// 		"Address" : "Vy5SKeKGXUHKS2WVpJ76HYuKAu3URastUo"
    /// 	}
    /// }
    /// ```
    pub fn get_deposit_address(&mut self, currency: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.private_query("/account/getdepositaddress", &mut params)
    }

    /// Used to withdraw funds from your account. note: please account for txfee.
    /// "currency" required a string literal for the currency (ie. BTC)
    /// "quantity" required the quantity of coins to withdraw
    /// "address" required the address where to send the funds.
    /// "paymentid" optional used for CryptoNotes/BitShareX/Nxt optional field (memo/paymentid)
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 			"uuid" : "68b5a16c-92de-11e3-ba3b-425861b86ab6"
    /// 	}
    /// }
    /// ```
    pub fn withdraw(&mut self, currency: &str, quantity: &str, address: &str, paymentid: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        params.insert("quantity", quantity);
        params.insert("address", address);
        params.insert("paymentid", paymentid);
        self.private_query("/account/withdraw", &mut params)
    }

    /// Used to retrieve a single order by uuid.
    /// "uuid" required the uuid of the buy or sell order
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : {
    /// 		"AccountId" : null,
    /// 		"OrderUuid" : "0cb4c4e4-bdc7-4e13-8c13-430e587d2cc1",
    /// 		"Exchange" : "BTC-SHLD",
    /// 		"Type" : "LIMIT_BUY",
    /// 		"Quantity" : 1000.00000000,
    /// 		"QuantityRemaining" : 1000.00000000,
    /// 		"Limit" : 0.00000001,
    /// 		"Reserved" : 0.00001000,
    /// 		"ReserveRemaining" : 0.00001000,
    /// 		"CommissionReserved" : 0.00000002,
    /// 		"CommissionReserveRemaining" : 0.00000002,
    /// 		"CommissionPaid" : 0.00000000,
    /// 		"Price" : 0.00000000,
    /// 		"PricePerUnit" : null,
    /// 		"Opened" : "2014-07-13T07:45:46.27",
    /// 		"Closed" : null,
    /// 		"IsOpen" : true,
    /// 		"Sentinel" : "6c454604-22e2-4fb4-892e-179eede20972",
    /// 		"CancelInitiated" : false,
    /// 		"ImmediateOrCancel" : false,
    /// 		"IsConditional" : false,
    /// 		"Condition" : "NONE",
    /// 		"ConditionTarget" : null
    /// 	}
    /// }
    /// ```
    pub fn get_order(&mut self, uuid: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("uuid", uuid);
        self.private_query("/account/getorder", &mut params)
    }

    /// Used to retrieve your order history.
    /// "market" optional a string literal for the market (ie. BTC-LTC).
    /// If ommited, will return for all markets
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"OrderUuid" : "fd97d393-e9b9-4dd1-9dbf-f288fc72a185",
    /// 			"Exchange" : "BTC-LTC",
    /// 			"TimeStamp" : "2014-07-09T04:01:00.667",
    /// 			"OrderType" : "LIMIT_BUY",
    /// 			"Limit" : 0.00000001,
    /// 			"Quantity" : 100000.00000000,
    /// 			"QuantityRemaining" : 100000.00000000,
    /// 			"Commission" : 0.00000000,
    /// 			"Price" : 0.00000000,
    /// 			"PricePerUnit" : null,
    /// 			"IsConditional" : false,
    /// 			"Condition" : null,
    /// 			"ConditionTarget" : null,
    /// 			"ImmediateOrCancel" : false
    /// 		}, {
    /// 			"OrderUuid" : "17fd64d1-f4bd-4fb6-adb9-42ec68b8697d",
    /// 			"Exchange" : "BTC-ZS",
    /// 			"TimeStamp" : "2014-07-08T20:38:58.317",
    /// 			"OrderType" : "LIMIT_SELL",
    /// 			"Limit" : 0.00002950,
    /// 			"Quantity" : 667.03644955,
    /// 			"QuantityRemaining" : 0.00000000,
    /// 			"Commission" : 0.00004921,
    /// 			"Price" : 0.01968424,
    /// 			"PricePerUnit" : 0.00002950,
    /// 			"IsConditional" : false,
    /// 			"Condition" : null,
    /// 			"ConditionTarget" : null,
    /// 			"ImmediateOrCancel" : false
    /// 		}
    /// 	]
    /// }
    /// ```
    pub fn get_order_history(&mut self, market: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("market", market);
        self.private_query("/account/getorderhistory", &mut params)
    }

    /// Used to retrieve your withdrawal history.
    /// "currency" optional	a string literal for the currecy (ie. BTC).
    /// If omitted, will return for all currencies
    /// 
    /// ```json
    /// {
	/// "success" : true,
	/// "message" : "",
	/// "result" : [{
	/// 		"PaymentUuid" : "b52c7a5c-90c6-4c6e-835c-e16df12708b1",
	/// 		"Currency" : "BTC",
	/// 		"Amount" : 17.00000000,
	/// 		"Address" : "1DeaaFBdbB5nrHj87x3NHS4onvw1GPNyAu",
	/// 		"Opened" : "2014-07-09T04:24:47.217",
	/// 		"Authorized" : true,
	/// 		"PendingPayment" : false,
	/// 		"TxCost" : 0.00020000,
	/// 		"TxId" : null,
	/// 		"Canceled" : true,
	/// 		"InvalidAddress" : false
	/// 	}, {
	/// 		"PaymentUuid" : "f293da98-788c-4188-a8f9-8ec2c33fdfcf",
	/// 		"Currency" : "XC",
	/// 		"Amount" : 7513.75121715,
	/// 		"Address" : "XVnSMgAd7EonF2Dgc4c9K14L12RBaW5S5J",
	/// 		"Opened" : "2014-07-08T23:13:31.83",
	/// 		"Authorized" : true,
	/// 		"PendingPayment" : false,
	/// 		"TxCost" : 0.00002000,
	/// 		"TxId" : "b4a575c2a71c7e56d02ab8e26bb1ef0a2f6cf2094f6ca2116476a569c1e84f6e",
	/// 		"Canceled" : false,
	/// 		"InvalidAddress" : false
	/// 	}
	///  ]
    /// }
    /// ```
    pub fn get_withdrawal_history(&mut self, currency: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.private_query("/account/getwithdrawalhistory", &mut params)
    }

    /// Used to retrieve your deposit history.
    /// "currency" optional a string literal for the currecy (ie. BTC).
    /// If omitted, will return for all currencies
    /// 
    /// ```json
    /// {
    /// 	"success" : true,
    /// 	"message" : "",
    /// 	"result" : [{
    /// 			"PaymentUuid" : "554ec664-8842-4fe9-b491-06225becbd59",
    /// 			"Currency" : "BTC",
    /// 			"Amount" : 0.00156121,
    /// 			"Address" : "1K37yQZaGrPKNTZ5KNP792xw8f7XbXxetE",
    /// 			"Opened" : "2014-07-11T03:41:25.323",
    /// 			"Authorized" : true,
    /// 			"PendingPayment" : false,
    /// 			"TxCost" : 0.00020000,
    /// 			"TxId" : "70cf6fdccb9bd38e1a930e13e4ae6299d678ed6902da710fa3cc8d164f9be126",
    /// 			"Canceled" : false,
    /// 			"InvalidAddress" : false
    /// 		}, {
    /// 			"PaymentUuid" : "d3fdf168-3d8e-40b6-8fe4-f46e2a7035ea",
    /// 			"Currency" : "BTC",
    /// 			"Amount" : 0.11800000,
    /// 			"Address" : "1Mrcar6715hjds34pdXuLqXcju6QgwHA31",
    /// 			"O
    /// 			pened" : "2014-07-03T20:27:07.163",
    /// 			"Authorized" : true,
    /// 			"PendingPayment" : false,
    /// 			"TxCost" : 0.00020000,
    /// 			"TxId" : "3efd41b3a051433a888eed3ecc174c1d025a5e2b486eb418eaaec5efddda22de",
    /// 			"Canceled" : false,
    /// 			"InvalidAddress" : false
    /// 		}
    ///     ]
    /// }
    /// ```
    pub fn get_deposit_history(&mut self, currency: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.private_query("/account/getdeposithistory", &mut params)
    }
}
