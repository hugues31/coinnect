//! Use this module to interact with Kraken exchange.
//! Please see examples for more informations.

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::{Sha256, Sha512};

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;

use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;
use std::str;
use std::iter::repeat;

use helpers;

header! { (KeyHeader, "API-Key") => [String] }
header! { (SignHeader, "API-Sign") => [String] }


pub struct KrakenApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
}


impl KrakenApi {
    /// Create a new KrakenApi by providing an API key & API secret
    pub fn new(api_key: &str, api_secret: &str) -> KrakenApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        KrakenApi {
            last_request: 0,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: Client::with_connector(connector),
        }
    }

    /// Create a new KrakenApi from a json configuration file. This file must follow this structure:
    ///
    /// ```ignore
    /// {
    ///     "account_kraken": {
    ///         "exchange"  : "kraken",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef"
    ///     },
    ///     "account_poloniex": {
    ///         "exchange"  : "poloniex",
    ///         "api_key"   : "XYXY-XYXY-XYXY-XY",
    ///         "api_secret": "A0A0B1B1C2C2"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Kraken account with
    /// `new_from_file("account_kraken", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> KrakenApi {
        let mut f = File::open(&path).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();
        let json_obj = data.as_object().unwrap().get(config_name).unwrap();
        let api_key = json_obj.get("api_key").unwrap().as_str().unwrap();
        let api_secret = json_obj.get("api_secret").unwrap().as_str().unwrap();

        KrakenApi::new(api_key, api_secret)
    }

    fn block_or_continue(&self) {
        let threshold = 2000; // 1 request/2sec
        let delay = helpers::get_unix_timestamp_ms() - self.last_request;
        if delay < threshold {
            let duration_ms = Duration::from_millis(delay as u64);
            thread::sleep(duration_ms);
        }
    }

    fn deserialize_json(&mut self, json_string: String) -> Option<Map<String, Value>> {
        let data: Value = serde_json::from_str(&json_string).unwrap();

        match data.as_object() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    fn public_query(&mut self, method: &str, params: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        let url = "https://api.kraken.com/0/public/".to_string() + method + "?" +
        &helpers::url_encode_hashmap(params);

        self.block_or_continue();
        let mut response = self.http_client.get(&url).send().unwrap();
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return self.deserialize_json(buffer);
    }

    fn private_query(&mut self, method: &str, params: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        let url = "https://api.kraken.com/0/private/".to_string() + method;

        let urlpath = "/0/private/".to_string() + method;

        // Create header
        let nonce = helpers::get_unix_timestamp_ms().to_string();
        let mut params = params.clone();

        params.insert("nonce", &nonce);

        let postdata = helpers::url_encode_hashmap(&params);
        let signature = self.create_signature(urlpath, &postdata, &nonce);

        let mut custom_header = header::Headers::new();
        custom_header.set(KeyHeader(self.api_key.clone()));
        custom_header.set(SignHeader(signature));

        let mut res = self.http_client
        .post(&url)
        .body(&postdata)
        .headers(custom_header)
        .send()
        .unwrap();

        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        return self.deserialize_json(buffer);
    }

    fn create_signature(&self, urlpath: String, postdata: &str, nonce: &str) -> String {
        let message_presha256 = nonce.to_string() + postdata;

        let mut sha256 = Sha256::new();
        sha256.input_str(&message_presha256);
        let mut buffer: Vec<u8> = repeat(0).take((sha256.output_bits()+7)/8).collect();
        sha256.result(&mut buffer);

        let mut concatenated = urlpath.as_bytes().to_vec();
        for elem in buffer {
            concatenated.push(elem);
        }

        let hmac_key = self.api_secret.from_base64().unwrap();
        let mut hmac = Hmac::new(Sha512::new(), &hmac_key);
        hmac.input(&concatenated);
        hmac.result().code().to_base64(STANDARD)
    }

    /// Result: Server's time
    ///
    /// ```ignore
    /// unixtime =  as unix timestamp
    /// rfc1123 = as RFC 1123 time format
    /// ```
    /// Note: This is to aid in approximating the skew time between the server and client.
    pub fn get_server_time(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("Time", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// info = info to retrieve (optional):
    ///     info = all info (default)
    /// aclass = asset class (optional):
    ///     currency (default)
    /// asset = comma delimited list of assets to get info on (optional.  default = all for given asset class)
    /// ```
    /// Result: array of asset names and their info:
    ///
    /// ```ignore
    /// <asset_name> = asset name
    /// altname = alternate name
    /// aclass = asset class
    /// decimals = scaling decimal places for record keeping
    /// display_decimals = scaling decimal places for output display
    /// ```
    pub fn get_asset_info(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("Assets", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// info = info to retrieve (optional):
    ///     info = all info (default)
    ///     leverage = leverage info
    ///     fees = fees schedule
    ///     margin = margin info
    /// pair = comma delimited list of asset pairs to get info on (optional.  default = all)
    /// ```
    ///
    /// Result: array of pair names and their info
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     altname = alternate pair name
    ///     aclass_base = asset class of base component
    ///     base = asset id of base component
    ///     aclass_quote = asset class of quote component
    ///     quote = asset id of quote component
    ///     lot = volume lot size
    ///     pair_decimals = scaling decimal places for pair
    ///     lot_decimals = scaling decimal places for volume
    ///     lot_multiplier = amount to multiply lot volume by to get currency volume
    ///     leverage_buy = array of leverage amounts available when buying
    ///     leverage_sell = array of leverage amounts available when selling
    ///     fees = fee schedule array in [volume, percent fee] tuples
    ///     fees_maker = maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    ///     fee_volume_currency = volume discount currency
    ///     margin_call = margin call level
    ///     margin_stop = stop-out/liquidation margin level
    /// ```
    pub fn get_tradable_asset_pairs(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.public_query("AssetPairs", &params)
    }

    /// Input: pair = comma delimited list of asset pairs to get info on
    /// Result: array of pair names and their ticker info
    ///
    /// ```ignore
    /// <pair_name> = pair name
    /// a = ask array(<price>, <whole lot volume>, <lot volume>),
    /// b = bid array(<price>, <whole lot volume>, <lot volume>),
    /// c = last trade closed array(<price>, <lot volume>),
    /// v = volume array(<today>, <last 24 hours>),
    /// p = volume weighted average price array(<today>, <last 24 hours>),
    /// t = number of trades array(<today>, <last 24 hours>),
    /// l = low array(<today>, <last 24 hours>),
    /// h = high array(<today>, <last 24 hours>),
    /// o = today's opening price
    /// ```
    pub fn get_ticker_information(&mut self, pair: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        self.public_query("Ticker", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get OHLC data for
    /// interval = time frame interval in minutes (optional):
    /// 	1 (default), 5, 15, 30, 60, 240, 1440, 10080, 21600
    /// since = return committed OHLC data since given id (optional.  exclusive)
    /// ```
    ///
    /// Result: array of pair name and OHLC data
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     array of array entries(<time>, <open>, <high>, <low>, <close>, <vwap>, <volume>, <count>)
    /// last = id to be used as since when polling for new, committed OHLC data
    /// ```
    ///
    /// Note: the last entry in the OHLC array is for the current, not-yet-committed frame and will always be present, regardless of the value of "since".
    pub fn get_ohlc_data(&mut self, pair: &str, interval: &str, since: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("interval", interval);
        params.insert("since", since);
        self.public_query("OHLC", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get market depth for
    /// count = maximum number of asks/bids (optional)
    /// ```
    /// Result: array of pair name and market depth
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     asks = ask side array of array entries(<price>, <volume>, <timestamp>)
    ///     bids = bid side array of array entries(<price>, <volume>, <timestamp>)
    /// ```
    pub fn get_order_book(&mut self, pair: &str, count: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("count", count);
        self.public_query("Depth", &params)
    }


    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get trade data for
    /// since = return trade data since given id (optional.  exclusive)
    /// ```
    /// Result: array of pair name and recent trade data
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     array of array entries(<price>, <volume>, <time>, <buy/sell>, <market/limit>, <miscellaneous>)
    /// last = id to be used as since when polling for new trade data
    /// ```
    pub fn get_recent_trades(&mut self, pair: &str, since: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("since", since);
        self.public_query("Trades", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = asset pair to get spread data for
    /// since = return spread data since given id (optional.  inclusive)
    /// ```
    ///
    /// Result: array of pair name and recent spread data
    ///
    /// ```ignore
    /// <pair_name> = pair name
    ///     array of array entries(<time>, <bid>, <ask>)
    /// last = id to be used as since when polling for new spread data
    /// ```
    /// Note: "since" is inclusive so any returned data with the same time as the previous set should overwrite all of the previous set's entries at that time
    pub fn get_recent_spread_data(&mut self, pair: &str, since: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("since", since);
        self.public_query("Spread", &params)
    }

    /// Result: array of asset names and balance amount
    pub fn get_account_balance(&mut self) -> Option<Map<String, Value>> {
        let params = HashMap::new();
        self.private_query("Balance", &params)
    }

    /// URL: https://api.kraken.com/0/private/TradeBalance
    ///
    /// Input:
    ///
    /// ```ignore
    /// aclass = asset class (optional):
    ///     currency (default)
    /// asset = base asset used to determine balance (default = ZUSD)
    /// ```
    /// Result: array of trade balance info
    ///
    /// ```ignore
    /// eb = equivalent balance (combined balance of all currencies)
    /// tb = trade balance (combined balance of all equity currencies)
    /// m = margin amount of open positions
    /// n = unrealized net profit/loss of open positions
    /// c = cost basis of open positions
    /// v = current floating valuation of open positions
    /// e = equity = trade balance + unrealized net profit/loss
    /// mf = free margin = equity - initial margin (maximum margin available to open new positions)
    /// ml = margin level = (equity / initial margin) * 100
    /// ```
    /// Note: Rates used for the floating valuation is the midpoint of the best bid and ask prices
    pub fn get_trade_balance(&mut self, aclass: &str, asset: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("aclass", aclass);
        params.insert("asset", asset);
        self.private_query("TradeBalance", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// trades = whether or not to include trades in output (optional.  default = false)
    /// userref = restrict results to given user reference id (optional)
    /// ```
    ///
    /// Result: array of order info in open array with txid as the key
    ///
    /// ```ignore
    /// refid = Referral order transaction id that created this order
    /// userref = user reference id
    /// status = status of order:
    ///     pending = order pending book entry
    ///     open = open order
    ///     closed = closed order
    ///     canceled = order canceled
    ///     expired = order expired
    /// opentm = unix timestamp of when order was placed
    /// starttm = unix timestamp of order start time (or 0 if not set)
    /// expiretm = unix timestamp of order end time (or 0 if not set)
    /// descr = order description info
    ///     pair = asset pair
    ///     type = type of order (buy/sell)
    ///     ordertype = order type (See Add standard order)
    ///     price = primary price
    ///     price2 = secondary price
    ///     leverage = amount of leverage
    ///     order = order description
    ///     close = conditional close order description (if conditional close set)
    /// vol = volume of order (base currency unless viqc set in oflags)
    /// vol_exec = volume executed (base currency unless viqc set in oflags)
    /// cost = total cost (quote currency unless unless viqc set in oflags)
    /// fee = total fee (quote currency)
    /// price = average price (quote currency unless viqc set in oflags)
    /// stopprice = stop price (quote currency, for trailing stops)
    /// limitprice = triggered limit price (quote currency, when limit based order type triggered)
    /// misc = comma delimited list of miscellaneous info
    ///     stopped = triggered by stop price
    ///     touched = triggered by touch price
    ///     liquidated = liquidation
    ///     partial = partial fill
    /// oflags = comma delimited list of order flags
    ///     viqc = volume in quote currency
    ///     fcib = prefer fee in base currency (default if selling)
    ///     fciq = prefer fee in quote currency (default if buying)
    ///     nompp = no market price protection
    /// trades = array of trade ids related to order (if trades info requested and data available)
    /// ```
    ///
    /// Note: Unless otherwise stated, costs, fees, prices, and volumes are in the asset pair's scale,
    /// not the currency's scale. For example, if the asset pair uses a lot size that has a scale of 8,
    /// the volume will use a scale of 8, even if the currency it represents only has a scale of 2.
    /// Similarly, if the asset pair's pricing scale is 5, the scale will remain as 5, even if the
    /// underlying currency has a scale of 8.
    pub fn get_open_orders(&mut self, trades: &str, userref: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("trades", trades);
        params.insert("userref", userref);
        self.private_query("OpenOrders", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// trades = whether or not to include trades in output (optional.  default = false)
    /// userref = restrict results to given user reference id (optional)
    /// start = starting unix timestamp or order tx id of results (optional.  exclusive)
    /// end = ending unix timestamp or order tx id of results (optional.  inclusive)
    /// ofs = result offset
    /// closetime = which time to use (optional)
    ///     open
    ///     close
    ///     both (default)
    /// ```
    ///
    /// Result: array of order info
    ///
    /// ```ignore
    /// closed = array of order info.  See Get open orders.  Additional fields:
    ///     closetm = unix timestamp of when order was closed
    ///     reason = additional info on status (if any)
    /// count = amount of available order info matching criteria
    /// ```
    /// Note: Times given by order tx ids are more accurate than unix timestamps. If an order tx id is given
    /// for the time, the order's open time is used
    pub fn get_closed_orders(&mut self, trades: &str, userref: &str, start: &str, end: &str, ofs: &str, closetime: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("trades", trades);
        params.insert("userref", userref);
        params.insert("start", start);
        params.insert("end", end);
        params.insert("ofs", ofs);
        params.insert("closetime", closetime);
        self.private_query("OpenOrders", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// trades = whether or not to include trades in output (optional.  default = false)
    /// userref = restrict results to given user reference id (optional)
    /// txid = comma delimited list of transaction ids to query info about (20 maximum)
    /// ```
    /// Result: associative array of orders info
    ///
    /// ```ignore
    /// <order_txid> = order info.  See Get open orders/Get closed orders
    /// ```
    pub fn query_orders_info(&mut self, trades: &str, userref: &str, txid: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("trades", trades);
        params.insert("userref", userref);
        params.insert("txid", txid);
        self.private_query("QueryOrders", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// type = type of trade (optional)
    ///     all = all types (default)
    ///     any position = any position (open or closed)
    ///     closed position = positions that have been closed
    ///     closing position = any trade closing all or part of a position
    ///     no position = non-positional trades
    /// trades = whether or not to include trades related to position in output (optional.  default = false)
    /// start = starting unix timestamp or trade tx id of results (optional.  exclusive)
    /// end = ending unix timestamp or trade tx id of results (optional.  inclusive)
    /// ofs = result offset
    /// ```
    /// Result: array of trade info
    ///
    /// ```ignore
    /// trades = array of trade info with txid as the key
    ///     ordertxid = order responsible for execution of trade
    ///     pair = asset pair
    ///     time = unix timestamp of trade
    ///     type = type of order (buy/sell)
    ///     ordertype = order type
    ///     price = average price order was executed at (quote currency)
    ///     cost = total cost of order (quote currency)
    ///     fee = total fee (quote currency)
    ///     vol = volume (base currency)
    ///     margin = initial margin (quote currency)
    ///     misc = comma delimited list of miscellaneous info
    ///         closing = trade closes all or part of a position
    /// count = amount of available trades info matching criteria
    /// If the trade opened a position, the follow fields are also present in the trade info:
    ///
    ///     posstatus = position status (open/closed)
    ///     cprice = average price of closed portion of position (quote currency)
    ///     ccost = total cost of closed portion of position (quote currency)
    ///     cfee = total fee of closed portion of position (quote currency)
    ///     cvol = total fee of closed portion of position (quote currency)
    ///     cmargin = total margin freed in closed portion of position (quote currency)
    ///     net = net profit/loss of closed portion of position (quote currency, quote currency scale)
    ///     trades = list of closing trades for position (if available)
    /// ```
    ///
    /// Note:
    ///
    /// Unless otherwise stated, costs, fees, prices, and volumes are in the asset pair's scale, not the currency's scale.
    /// Times given by trade tx ids are more accurate than unix timestamps.
    pub fn get_trades_history(&mut self, type_trade: &str, trades: &str, start: &str, end: &str, ofs: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("type", type_trade);
        params.insert("trades", trades);
        params.insert("start", start);
        params.insert("end", end);
        params.insert("ofs", ofs);
        self.private_query("TradesHistory", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// txid = comma delimited list of transaction ids to query info about (20 maximum)
    /// trades = whether or not to include trades related to position in output (optional.  default = false)
    /// ```
    // Result: associative array of trades info
    ///
    /// ```ignore
    /// <trade_txid> = trade info.  See Get trades history
    /// ```
    pub fn query_trades_info(&mut self, txid: &str, trades: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("txid", txid);
        params.insert("trades", trades);
        self.private_query("QueryTrades", &params)
    }
    /// Input:
    ///
    /// ```ignore
    /// txid = comma delimited list of transaction ids to restrict output to
    /// docalcs = whether or not to include profit/loss calculations (optional.  default = false)
    /// ```
    /// Result: associative array of open position info
    ///
    /// ```ignore
    /// <position_txid> = open position info
    ///     ordertxid = order responsible for execution of trade
    ///     pair = asset pair
    ///     time = unix timestamp of trade
    ///     type = type of order used to open position (buy/sell)
    ///     ordertype = order type used to open position
    ///     cost = opening cost of position (quote currency unless viqc set in oflags)
    ///     fee = opening fee of position (quote currency)
    ///     vol = position volume (base currency unless viqc set in oflags)
    ///     vol_closed = position volume closed (base currency unless viqc set in oflags)
    ///     margin = initial margin (quote currency)
    ///     value = current value of remaining position (if docalcs requested.  quote currency)
    ///     net = unrealized profit/loss of remaining position (if docalcs requested.  quote
    ///           currency, quote currency scale)
    ///     misc = comma delimited list of miscellaneous info
    ///     oflags = comma delimited list of order flags
    ///         viqc = volume in quote currency
    /// ```
    ///
    /// Note: Unless otherwise stated, costs, fees, prices, and volumes are in the asset pair's
    /// scale, not the currency's scale.
    pub fn get_open_positions(&mut self, txid: &str, docalcs: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("txid", txid);
        params.insert("docalcs", docalcs);
        self.private_query("OpenPositions", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// aclass = asset class (optional):
    ///     currency (default)
    /// asset = comma delimited list of assets to restrict output to (optional.  default = all)
    /// type = type of ledger to retrieve (optional):
    ///     all (default)
    ///     deposit
    ///     withdrawal
    ///     trade
    ///     margin
    /// start = starting unix timestamp or ledger id of results (optional.  exclusive)
    /// end = ending unix timestamp or ledger id of results (optional.  inclusive)
    /// ofs = result offset
    /// ```
    /// Result: associative array of ledgers info
    ///
    /// ```ignore
    /// <ledger_id> = ledger info
    ///     refid = reference id
    ///     time = unx timestamp of ledger
    ///     type = type of ledger entry
    ///     aclass = asset class
    ///     asset = asset
    ///     amount = transaction amount
    ///     fee = transaction fee
    ///     balance = resulting balance
    /// ```
    /// Note: Times given by ledger ids are more accurate than unix timestamps.
    pub fn get_ledgers_info(&mut self, aclass: &str, asset: &str, type_ledger: &str, start: &str, end: &str, ofs: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("aclass", aclass);
        params.insert("asset", asset);
        params.insert("type_ledger", type_ledger);
        params.insert("start", start);
        params.insert("end", end);
        params.insert("ofs", ofs);
        self.private_query("Ledgers", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// id = comma delimited list of ledger ids to query info about (20 maximum)
    /// ```
    /// Result: associative array of ledgers info
    ///
    /// ```ignore
    /// <ledger_id> = ledger info.  See Get ledgers info
    /// ```
    pub fn query_ledgers(&mut self, id: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("id", id);
        self.private_query("QueryLedgers", &params)
    }

    /// Input:
    ///
    /// ```ignore
    /// pair = comma delimited list of asset pairs to get fee info on (optional)
    /// fee-info = whether or not to include fee info in results (optional)
    /// ```
    /// Result: associative array
    ///
    /// ```ignore
    /// currency = volume currency
    /// volume = current discount volume
    /// fees = array of asset pairs and fee tier info (if requested)
    ///     fee = current fee in percent
    ///     minfee = minimum fee for pair (if not fixed fee)
    ///     maxfee = maximum fee for pair (if not fixed fee)
    ///     nextfee = next tier's fee for pair (if not fixed fee.  nil if at lowest fee tier)
    ///     nextvolume = volume level of next tier (if not fixed fee.  nil if at lowest fee tier)
    ///     tiervolume = volume level of current tier (if not fixed fee.  nil if at lowest fee tier)
    /// fees_maker = array of asset pairs and maker fee tier info (if requested) for any pairs on
    ///             maker/taker schedule
    ///     fee = current fee in percent
    ///     minfee = minimum fee for pair (if not fixed fee)
    ///     maxfee = maximum fee for pair (if not fixed fee)
    ///     nextfee = next tier's fee for pair (if not fixed fee.  nil if at lowest fee tier)
    ///     nextvolume = volume level of next tier (if not fixed fee.  nil if at lowest fee tier)
    ///     tiervolume = volume level of current tier (if not fixed fee.  nil if at lowest fee tier)
    /// ```
    /// Note: If an asset pair is on a maker/taker fee schedule, the taker side is given in "fees"
    /// and maker side in "fees_maker". For pairs not on maker/taker, they will only be given in
    /// "fees".
    pub fn get_trade_volume(&mut self, pair: &str, fee_info: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("pair", pair);
        params.insert("fee-info", fee_info);
        self.private_query("TradeVolume", &params)
    }
}
