//! Use this module to interact with Poloniex exchange.
//! See examples for more informations.

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;

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

use error::*;
use helpers;

use exchange::Exchange;
use coinnect::Credentials;
use poloniex::utils;

header! {
    #[doc(hidden)]
    (KeyHeader, "Key") => [String]
}

header! {
    #[doc(hidden)]
    (SignHeader, "Sign") => [String]
}

header! {
    #[doc(hidden)]
    (ContentHeader, "Content-Type") => [String]
}

#[derive(Debug, Copy, Clone)]
pub enum PlaceOrderOption {
    FillOrKill,
    ImmediateOrCancel,
    PostOnly,
}
impl PlaceOrderOption {
    fn repr(&self) -> &'static str {
        match self {
            &PlaceOrderOption::FillOrKill => "fillOrKill",
            &PlaceOrderOption::ImmediateOrCancel => "immediateOrCancel",
            &PlaceOrderOption::PostOnly => "postOnly",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MoveOrderOption {
    ImmediateOrCancel,
    PostOnly,
}
impl MoveOrderOption {
    fn repr(&self) -> &'static str {
        match self {
            &MoveOrderOption::ImmediateOrCancel => "immediateOrCancel",
            &MoveOrderOption::PostOnly => "postOnly",
        }
    }
}

#[derive(Debug)]
pub struct PoloniexApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
    burst: bool,
}

impl PoloniexApi {
    /// Create a new PoloniexApi by providing an API key & API secret
    pub fn new<C: Credentials>(creds: C) -> Result<PoloniexApi> {
        if creds.exchange() != Exchange::Poloniex {
            return Err(ErrorKind::InvalidConfigType(Exchange::Poloniex, creds.exchange()).into());
        }

        //TODO: Handle correctly the TLS errors with error_chain.
        let ssl = match NativeTlsClient::new() {
            Ok(res) => res,
            Err(_) => return Err(ErrorKind::TlsError.into()),
        };
        let connector = HttpsConnector::new(ssl);

        Ok(PoloniexApi {
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

    fn block_or_continue(&self) {
        if !self.burst {
            let threshold: u64 = 167; // 6 requests/sec = 1/6*1000
            let offset: u64 = helpers::get_unix_timestamp_ms() as u64 - self.last_request as u64;
            if offset < threshold {
                let wait_ms = Duration::from_millis(threshold - offset);
                thread::sleep(wait_ms);
            }
        }
    }

    fn public_query(&mut self, method: &str, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {
        let mut params = params.clone();
        helpers::strip_empties(&mut params);
        let url = "https://poloniex.com/public?command=".to_string() + method + "&" + &helpers::url_encode_hashmap(&params);

        self.block_or_continue();
        let mut response = match self.http_client.get(&url).send() {
            Ok(response) => response,
            Err(err) => return Err(ErrorKind::ServiceUnavailable(err.to_string()).into()),
        };
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;

        if method == "returnChartData" {
            return utils::deserialize_json_array(&buffer);
        }
        utils::deserialize_json(&buffer)
    }

    fn private_query(&mut self, method: &str, params: &HashMap<&str, &str>) -> Result<Map<String, Value>> {
        let unix_timestamp = helpers::get_unix_timestamp_us().to_string();
        let mut post_params = params.clone();
        post_params.insert("command", method);
        post_params.insert("nonce", &unix_timestamp);
        helpers::strip_empties(&mut post_params);
        let post_data = helpers::url_encode_hashmap(&post_params);

        let mut mac = Hmac::<Sha512>::new_from_slice(self.api_secret.as_bytes()).unwrap();
        mac.update(post_data.as_bytes());

        let sign = HEXLOWER.encode(&mac.finalize().into_bytes());

        let mut custom_header = header::Headers::new();
        custom_header.set(KeyHeader(self.api_key.to_owned()));
        custom_header.set(SignHeader(sign));
        custom_header.set(ContentHeader(
            "application/x-www-form-urlencoded".to_owned(),
        ));

        self.block_or_continue();

        let mut response = match self.http_client
            .post("https://poloniex.com/tradingApi")
            .body(&post_data)
            .headers(custom_header)
            .send()
        {
            Ok(response) => response,
            Err(err) => return Err(ErrorKind::ServiceUnavailable(err.to_string()).into()),
        };
        self.last_request = helpers::get_unix_timestamp_ms();

        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        if method == "returnChartData" {
            return utils::deserialize_json_array(&buffer);
        }
        utils::deserialize_json(&buffer)
    }

    /// Sample output :
    ///
    /// ```json
    /// {
    /// "BTC_LTC":{
    /// "last":"0.0251","lowestAsk":"0.02589999","highestBid":"0.0251",
    /// "percentChange":"0.02390438","baseVolume":"6.16485315","quoteVolume":"245.82513926"},
    /// "BTC_NXT":{
    /// "last":"0.00005730","lowestAsk":"0.00005710","highestBid":"0.00004903",
    /// "percentChange":"0.16701570","baseVolume":"0.45347489","quoteVolume":"9094"},
    /// ... }
    /// ```
    pub fn return_ticker(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("returnTicker", &params)
    }

    /// Sample output :
    ///
    /// ```json
    /// {"BTC_LTC":{"BTC":"2.23248854","LTC":"87.10381314"},"BTC_NXT":{"BTC":"0.981616",
    /// "NXT":"14145"},
    /// ... "totalBTC":"81.89657704","totalLTC":"78.52083806"}
    /// ```
    pub fn return_24_volume(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("return24Volume", &params)
    }

    /// Sample output :
    ///
    /// ```json
    /// {"asks":[[0.00007600,1164],[0.00007620,1300], ... ], "bids":[[0.00006901,200],
    /// [0.00006900,408], ... ], "isFrozen": 0, "seq": 18849}
    /// ```
    pub fn return_order_book(&mut self, currency_pair: &str, depth: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("depth", depth);
        self.public_query("returnOrderBook", &params)
    }

    /// Sample output :
    ///
    /// ```json
    /// [{"date":"2014-02-10 04:23:23","type":"buy","rate":"0.00007600","amount":"140",
    /// "total":"0.01064"},
    /// {"date":"2014-02-10 01:19:37","type":"buy","rate":"0.00007600","amount":"655",
    /// "total":"0.04978"}, ... ]
    /// ```
    pub fn return_trade_history(&mut self, currency_pair: &str, start: &str, end: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("start", start);
        params.insert("end", end);
        self.public_query("returnTradeHistory", &params)
    }

    /// Sample output :
    ///
    /// ```json
    /// {"data": {"date":1405699200,"high":0.0045388,"low":0.00403001,"open":0.00404545,"close":0.00427592,
    /// "volume":44.11655644,"quoteVolume":10259.29079097,"weightedAverage":0.00430015}, ...}
    /// ```
    pub fn return_chart_data(&mut self, currency_pair: &str, start: &str, end: &str, period: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("start", start);
        params.insert("end", end);
        params.insert("period", period);
        self.public_query("returnChartData", &params)
    }

    /// Sample output :
    ///
    /// ```json
    /// {"1CR":{"maxDailyWithdrawal":10000,"txFee":0.01,"minConf":3,"disabled":0},
    /// "ABY":{"maxDailyWithdrawal":10000000,"txFee":0.01,"minConf":8,"disabled":0}, ... }
    /// ```
    pub fn return_currencies(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("returnCurrencies", &params)
    }

    /// Sample output :
    ///
    /// ```json
    /// {"offers":[{"rate":"0.00200000","amount":"64.66305732","rangeMin":2,"rangeMax":8}, ... ],
    /// "demands":[{"rate":"0.00170000","amount":"26.54848841","rangeMin":2,"rangeMax":2}, ... ]}
    /// ```
    pub fn return_loan_orders(&mut self, currency: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.public_query("returnLoanOrders", &params)
    }

    /// Returns all of your available balances.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"BTC":"0.59098578","LTC":"3.31117268", ... }
    /// ```
    pub fn return_balances(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnBalances", &params)
    }

    /// Returns all of your balances, including available balance, balance on orders,
    /// and the estimated BTC value of your balance. By default, this call is limited to your
    /// exchange account; set the "account" POST parameter to "all" to include your margin and
    /// lending accounts.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"LTC":{"available":"5.015","onOrders":"1.0025","btcValue":"0.078"},"NXT":{...}, ... }
    /// ```
    pub fn return_complete_balances(&mut self) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("account", "all");
        self.private_query("returnCompleteBalances", &params)
    }

    /// Returns all of your deposit addresses.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"BTC":"19YqztHmspv2egyD6jQM3yn81x5t5krVdJ","LTC":"LPgf9kjv9H1Vuh4XSaKhzBe8JHdou1WgUB",
    /// ... "ITC":"Press Generate.." ... }
    /// ```
    pub fn return_deposit_addresses(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnDepositAddresses", &params)
    }

    /// Generates a new deposit address for the currency specified by the "currency" POST parameter.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"response":"CKXbbs8FAVbtEa397gJHSutmrdrBrhUMxe"}
    /// ```
    pub fn generate_new_address(&mut self, currency: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.private_query("generateNewAddress", &params)
    }

    /// Returns your deposit and withdrawal history within a range, specified by the "start" and
    /// "end" POST parameters,
    /// both of which should be given as UNIX timestamps.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"deposits":
    /// [{"currency":"BTC","address":"...","amount":"0.01006132","confirmations":10,
    /// "txid":"17f819a91369a9ff6c4a34216d434597cfc1b4a3d0489b46bd6f924137a47701",
    /// "timestamp":1399305798,"status":"COMPLETE"},
    /// {"currency":"BTC","address":"...","amount":"0.00404104","confirmations":10,
    /// "txid":"7acb90965b252e55a894b535ef0b0b65f45821f2899e4a379d3e43799604695c",
    /// "timestamp":1399245916,"status":"COMPLETE"}],
    /// "withdrawals":[{"withdrawalNumber":134933,"currency":"BTC",
    /// "address":"1N2i5n8DwTGzUq2Vmn9TUL8J1vdr1XBDFg","amount":"5.00010000",
    /// "timestamp":1399267904,
    /// "status":"COMPLETE: 36e483efa6aff9fd53a235177579d98451c4eb237c210e66cd2b9a2d4a988f8e",
    /// "ipAddress":"..."}]}
    /// ```
    pub fn return_deposits_withdrawals(&mut self, start: &str, end: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("start", start);
        params.insert("end", end);
        self.private_query("returnDepositsWithdrawals", &params)
    }

    ///Returns your open orders for a given market, specified by the "currencyPair" POST parameter,
    /// e.g. "BTC_XCP". Set "currencyPair" to "all" to return open orders for all markets.
    ///
    /// Sample output for single market:
    ///
    /// ```json
    /// [{"orderNumber":"120466","type":"sell","rate":"0.025","amount":"100","total":"2.5"},
    /// {"orderNumber":"120467","type":"sell","rate":"0.04","amount":"100","total":"4"}, ... ]
    /// ```
    ///
    /// ```json
    /// Or, for all markets:
    /// {"BTC_1CR":[],"BTC_AC":[{"orderNumber":"120466","type":"sell","rate":"0.025",
    /// "amount":"100","total":"2.5"},
    /// {"orderNumber":"120467","type":"sell","rate":"0.04","amount":"100","total":"4"}], ... }
    /// ```
    pub fn return_open_orders(&mut self, currency_pair: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        self.private_query("returnOpenOrders", &params)
    }

    /// Returns your trade history for a given market, specified by the "currencyPair" POST
    /// parameter.
    /// You may specify "all" as the currencyPair to receive your trade history for all markets.
    /// You may optionally specify a range via "start" and/or "end" POST parameters, given in UNIX
    /// timestamp format;
    /// if you do not specify a range, it will be limited to one day.
    ///
    /// Sample output:
    ///
    /// ```json
    /// [{ "globalTradeID": 25129732, "tradeID": "6325758", "date": "2016-04-05 08:08:40",
    /// "rate": "0.02565498", "amount": "0.10000000", "total": "0.00256549", "fee": "0.00200000",
    /// "orderNumber": "34225313575", "type": "sell", "category": "exchange" },
    /// { "globalTradeID": 25129628, "tradeID": "6325741", "date": "2016-04-05 08:07:55",
    /// "rate": "0.02565499", "amount": "0.10000000", "total": "0.00256549",
    /// "fee": "0.00200000", "orderNumber": "34225195693", "type": "buy", "category": "exchange" },
    /// ... ]
    /// ```
    ///
    /// Or, for all markets:
    ///
    /// ```json
    /// {"BTC_MAID": [ { "globalTradeID": 29251512, "tradeID": "1385888",
    /// "date": "2016-05-03 01:29:55", "rate": "0.00014243", "amount": "353.74692925",
    /// "total": "0.05038417", "fee": "0.00200000", "orderNumber": "12603322113", "type": "buy",
    /// "category": "settlement" },
    /// { "globalTradeID": 29251511, "tradeID": "1385887", "date": "2016-05-03 01:29:55",
    /// "rate": "0.00014111", "amount": "311.24262497", "total": "0.04391944", "fee": "0.00200000",
    /// "orderNumber": "12603319116", "type": "sell", "category": "marginTrade" }, ... ],
    /// "BTC_LTC":[ ... ] ... }
    /// ```
    pub fn return_private_trade_history(&mut self, currency_pair: &str, start: &str, end: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("start", start);
        params.insert("end", end);
        self.private_query("returnTradeHistory", &params)
    }

    /// Returns all trades involving a given order, specified by the "orderNumber" POST parameter.
    /// If no trades for the order have occurred or you specify an order that does not belong to
    /// you, you will receive an error.
    ///
    /// Sample output:
    ///
    /// ```json
    /// [{"globalTradeID": 20825863, "tradeID": 147142, "currencyPair": "BTC_XVC", "type": "buy",
    /// "rate": "0.00018500", "amount": "455.34206390", "total": "0.08423828", "fee": "0.00200000",
    /// "date": "2016-03-14 01:04:36"}, ...]
    /// ```
    pub fn return_order_trades(&mut self, order_number: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("orderNumber", order_number);
        self.private_query("returnOrderTrades", &params)
    }

    /// Places a limit buy order in a given market. Required POST parameters are "currencyPair",
    /// "rate", and "amount".
    /// If successful, the method will return the order number.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"orderNumber":31226040,"resultingTrades":[{"amount":"338.8732",
    /// "date":"2014-10-18 23:03:21", "rate":"0.00000173","total":"0.00058625","tradeID":"16164",
    /// "type":"buy"}]}
    /// ```
    pub fn buy<O>(&mut self, currency_pair: &str, rate: &str, amount: &str, option: O) -> Result<Map<String, Value>>
    where
        O: Into<Option<PlaceOrderOption>>,
    {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("rate", rate);
        params.insert("amount", amount);
        option.into().map(|o| params.insert(o.repr(), "1"));
        self.private_query("buy", &params)
    }

    /// Places a sell order in a given market. Parameters and output are the same as for the buy
    /// method.
    pub fn sell<O>(&mut self, currency_pair: &str, rate: &str, amount: &str, option: O) -> Result<Map<String, Value>>
    where
        O: Into<Option<PlaceOrderOption>>,
    {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("rate", rate);
        params.insert("amount", amount);
        option.into().map(|o| params.insert(o.repr(), "1"));
        self.private_query("sell", &params)
    }

    /// Cancels an order you have placed in a given market.
    /// Required POST parameter is "orderNumber". If successful, the method will return:
    /// {"success":1}
    pub fn cancel_order(&mut self, order_number: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("orderNumber", order_number);
        self.private_query("cancelOrder", &params)
    }

    /// Cancels an order and places a new one of the same type in a single atomic transaction,
    /// meaning either both operations will succeed or both will fail.
    /// Required POST parameters are "orderNumber" and "rate"; you may optionally
    /// specify "amount" if you wish to change the amount of the new order.
    /// "postOnly" or "immediateOrCancel" may be specified for exchange orders, but will have no
    /// effect on margin orders.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"orderNumber":"239574176","resultingTrades":{"BTC_BTS":[]}}
    /// ```
    pub fn move_order<O>(&mut self, order_number: &str, rate: &str, option: O) -> Result<Map<String, Value>>
    where
        O: Into<Option<MoveOrderOption>>,
    {
        // TODO: add optional parameters
        let mut params = HashMap::new();
        params.insert("orderNumber", order_number);
        params.insert("rate", rate);
        option.into().map(|o| params.insert(o.repr(), "1"));
        self.private_query("moveOrder", &params)
    }

    /// Immediately places a withdrawal for a given currency, with no email confirmation.
    /// In order to use this method, the withdrawal privilege must be enabled for your API key.
    /// Required POST parameters are "currency", "amount", and "address".
    /// For XMR withdrawals, you may optionally specify "paymentId".
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"response":"Withdrew 2398 NXT."}
    /// ```
    pub fn withdraw(&mut self, currency: &str, amount: &str, address: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        params.insert("amount", amount);
        params.insert("address", address);
        self.private_query("withdraw", &params)
    }

    /// If you are enrolled in the maker-taker fee schedule, returns your current
    /// trading fees and trailing 30-day volume in BTC. This information is updated once every
    /// 24 hours.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"makerFee": "0.00140000", "takerFee": "0.00240000", "thirtyDayVolume": "612.00248891",
    /// "nextTier": "1200.00000000"}
    /// ```
    pub fn return_free_info(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnFeeInfo", &params)
    }

    /// Returns your balances sorted by account. You may optionally specify the "account" POST
    /// parameter if you wish to fetch only the balances of one account. Please note that balances
    /// in your margin account may not be accessible if you have any open margin positions or
    /// orders.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"exchange":{"BTC":"1.19042859","BTM":"386.52379392","CHA":"0.50000000",
    /// "DASH":"120.00000000", "STR":"3205.32958001", "VNL":"9673.22570147"},
    /// "margin":{"BTC":"3.90015637", "DASH":"250.00238240","XMR":"497.12028113"},
    /// "lending":{"DASH":"0.01174765","LTC":"11.99936230"}}
    /// ```
    pub fn return_available_account_balances(&mut self, account: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("account", account);
        self.private_query("returnAvailableAccountBalances", &params)
    }

    /// Returns your current tradable balances for each currency in each market for which
    /// margin trading is enabled. Please note that these balances may vary continually with
    /// market conditions.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"BTC_DASH":{"BTC":"8.50274777","DASH":"654.05752077"},"BTC_LTC":{"BTC":"8.50274777",
    /// "LTC":"1214.67825290"},"BTC_XMR":{"BTC":"8.50274777","XMR":"3696.84685650"}}
    /// ```
    pub fn return_tradable_balances(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnTradableBalances", &params)
    }

    /// Transfers funds from one account to another (e.g. from your exchange account to your
    /// margin account). Required POST parameters are "currency", "amount", "fromAccount",
    /// and "toAccount".
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":"Transferred 2 BTC from exchange to margin account."}
    /// ```
    pub fn transfer_balance(&mut self, currency: &str, amount: &str, from_account: &str, to_account: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        params.insert("amount", amount);
        params.insert("fromAccount", from_account);
        params.insert("toAccount", to_account);
        self.private_query("transferBalance", &params)
    }

    /// Returns a summary of your entire margin account. This is the same information you will
    /// find in the Margin Account section of the Margin Trading page, under the Markets list.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"totalValue": "0.00346561","pl": "-0.00001220","lendingFees": "0.00000000",
    /// "netValue": "0.00345341","totalBorrowedValue": "0.00123220","currentMargin": "2.80263755"}
    /// ```
    pub fn return_margin_account_summary(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnMarginAccountSummary", &params)
    }

    /// Places a margin buy order in a given market. Required POST parameters are
    /// "currencyPair", "rate", and "amount". You may optionally specify a maximum lending
    /// rate using the "lendingRate" parameter. If successful, the method will return the order
    /// number and any trades immediately resulting from your order.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":"Margin order placed.","orderNumber":"154407998",
    /// "resultingTrades":{"BTC_DASH":[{"amount":"1.00000000","date":"2015-05-10 22:47:05",
    /// "rate":"0.01383692","total":"0.01383692","tradeID":"1213556","type":"buy"}]}}
    /// ```
    pub fn margin_buy(&mut self, currency_pair: &str, rate: &str, amount: &str, lending_rate: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("rate", rate);
        params.insert("amount", amount);
        params.insert("lendingRate", lending_rate);
        self.private_query("marginBuy", &params)
    }

    /// Places a margin sell order in a given market. Required POST parameters are
    /// "currencyPair", "rate", and "amount". You may optionally specify a maximum lending
    /// rate using the "lendingRate" parameter. If successful, the method will return the order
    /// number and any trades immediately resulting from your order.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":"Margin order placed.","orderNumber":"154407998",
    /// "resultingTrades":{"BTC_DASH":[{"amount":"1.00000000","date":"2015-05-10 22:47:05",
    /// "rate":"0.01383692","total":"0.01383692","tradeID":"1213556","type":"sell"}]}}
    /// ```
    pub fn margin_sell(&mut self, currency_pair: &str, rate: &str, amount: &str, lending_rate: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("rate", rate);
        params.insert("amount", amount);
        params.insert("lendingRate", lending_rate);
        self.private_query("marginSell", &params)
    }

    /// Returns information about your margin position in a given market, specified by the
    /// "currencyPair" POST parameter. You may set "currencyPair" to "all" if you wish to fetch all
    /// of your margin positions at once. If you have no margin position in the specified market,
    /// "type" will be set to "none". "liquidationPrice" is an estimate, and does not necessarily
    /// represent the price at which an actual forced liquidation will occur. If you have no
    /// liquidation price, the value will be -1.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"amount":"40.94717831","total":"-0.09671314","basePrice":"0.00236190",
    /// "liquidationPrice":-1,"pl":"-0.00058655", "lendingFees":"-0.00000038","type":"long"}
    /// ```
    pub fn get_margin_position(&mut self, currency_pair: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        self.private_query("getMarginPosition", &params)
    }

    /// Closes your margin position in a given market (specified by the "currencyPair" POST
    /// parameter) using a market order. This call will also return success if you do not have an
    /// open position in the specified market.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":"Successfully closed margin position.",
    /// "resultingTrades":{"BTC_XMR":[{"amount":"7.09215901","date":"2015-05-10 22:38:49",
    /// "rate":"0.00235337","total":"0.01669047","tradeID":"1213346","type":"sell"},
    /// {"amount":"24.00289920","date":"2015-05-10 22:38:49","rate":"0.00235321",
    /// "total":"0.05648386","tradeID":"1213347","type":"sell"}]}}
    /// ```
    pub fn close_margin_position(&mut self, currency_pair: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        self.private_query("closeMarginPosition", &params)
    }

    /// Creates a loan offer for a given currency. Required POST parameters are "currency",
    /// "amount", "duration", "autoRenew" (0 or 1), and "lendingRate".
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":"Loan order placed.","orderID":10590}
    /// ```
    pub fn create_loan_offer(&mut self, currency: &str, amount: &str, duration: &str, auto_renew: &str, lending_rate: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        params.insert("amount", amount);
        params.insert("duration", duration);
        params.insert("autoRenew", auto_renew);
        params.insert("lendingRate", lending_rate);
        self.private_query("createLoanOffer", &params)
    }

    /// Cancels a loan offer specified by the "orderNumber" POST parameter.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":"Loan offer canceled."}
    /// ```
    pub fn cancel_loan_offer(&mut self, order_number: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("orderNumber", order_number);
        self.private_query("cancelLoanOffer", &params)
    }

    /// Returns your open loan offers for each currency.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"BTC":[{"id":10595,"rate":"0.00020000","amount":"3.00000000","duration":2,"autoRenew":1,
    /// "date":"2015-05-10 23:33:50"}],"LTC":[{"id":10598,"rate":"0.00002100",
    /// "amount":"10.00000000","duration":2,"autoRenew":1,"date":"2015-05-10 23:34:35"}]}
    /// ```
    pub fn return_open_loan_offers(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnOpenLoanOffers", &params)
    }

    /// Returns your active loans for each currency.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"provided":[{"id":75073,"currency":"LTC","rate":"0.00020000","amount":"0.72234880",
    /// "range":2,"autoRenew":0,"date":"2015-05-10 23:45:05","fees":"0.00006000"},
    /// {"id":74961,"currency":"LTC","rate":"0.00002000","amount":"4.43860711","range":2,
    /// "autoRenew":0,"date":"2015-05-10 23:45:05","fees":"0.00006000"}],
    /// "used":[{"id":75238,"currency":"BTC","rate":"0.00020000","amount":"0.04843834","range":2,
    /// "date":"2015-05-10 23:51:12","fees":"-0.00000001"}]}
    /// ```
    pub fn return_active_loans(&mut self) -> Result<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("returnActiveLoans", &params)
    }

    /// Returns your lending history within a time range specified by the "start" and "end" POST
    /// parameters as UNIX timestamps. "limit" may also be specified to limit the number of rows
    /// returned.
    ///
    /// Sample output:
    ///
    /// ```json
    /// [{ "id": 175589553, "currency": "BTC", "rate": "0.00057400", "amount": "0.04374404",
    /// "duration": "0.47610000", "interest": "0.00001196", "fee": "-0.00000179",
    /// "earned": "0.00001017", "open": "2016-09-28 06:47:26", "close": "2016-09-28 18:13:03" }]
    /// ```
    pub fn return_lending_history(&mut self, start: &str, end: &str, limit: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("start", start);
        params.insert("end", end);
        params.insert("limit", limit);
        self.private_query("returnLendingHistory", &params)
    }

    /// Toggles the autoRenew setting on an active loan, specified by the "orderNumber" POST
    /// parameter. If successful, "message" will indicate the new autoRenew setting.
    ///
    /// Sample output:
    ///
    /// ```json
    /// {"success":1,"message":0}
    /// ```
    pub fn toggle_auto_renew(&mut self, order_number: &str) -> Result<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("orderNumber", order_number);
        self.private_query("toggleAutoRenew", &params)
    }
}

#[cfg(test)]
mod poloniex_api_tests {
    use super::*;

    #[test]
    fn should_block_or_not_block_when_enabled_or_disabled() {
        let mut api = PoloniexApi {
            last_request: helpers::get_unix_timestamp_ms(),
            api_key: "".to_string(),
            api_secret: "".to_string(),
            http_client: Client::new(),
            burst: false,
        };

        let mut counter = 0;
        loop {
            api.set_burst(false);
            let start = helpers::get_unix_timestamp_ms();
            api.block_or_continue();
            api.last_request = helpers::get_unix_timestamp_ms();

            let difference = api.last_request - start;
            assert!(difference >= 166);
            assert!(difference < 1000);

            api.set_burst(true);
            let start = helpers::get_unix_timestamp_ms();
            api.block_or_continue();
            api.last_request = helpers::get_unix_timestamp_ms();

            let difference = api.last_request - start;
            assert!(difference < 10);

            counter = counter + 1;
            if counter >= 3 {
                break;
            }
        }
    }
}
