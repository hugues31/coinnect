use bidir_map::BidirMap;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::thread;
use std::time::Duration;

use error::*;
use helpers;
use pair::Pair;
use pair::Pair::*;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(BTC_USD, "btcusd");
        m.insert(BTC_EUR, "btceur");
        m.insert(EUR_USD, "eurusd");
        m.insert(XRP_USD, "xrpusd");
        m.insert(XRP_EUR, "xrpeur");
        m.insert(XRP_BTC, "xrpbtc");
        m
    };
}

/// Return the name associated to pair used by Bitstamp
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: &Pair) -> Option<&&str> {
    PAIRS_STRING.get_by_first(pair)
}

/// Return the Pair enum associated to the string used by Bitstamp
/// If the Pair is not supported, None is returned.
pub fn get_pair_enum(pair: &str) -> Option<&Pair> {
    PAIRS_STRING.get_by_second(&pair)
}

pub fn block_or_continue(last_request: i64) {
    let threshold = 1000; // 600 requests per 10 mins = 1 request per second
    let delay = helpers::get_unix_timestamp_ms() - last_request;
    if delay < threshold {
        let duration_ms = Duration::from_millis(delay as u64);
        thread::sleep(duration_ms);
    }
}

pub fn build_signature(nonce: &str,
                       customer_id: &str,
                       api_key: &str,
                       api_secret: &str)
                       -> Result<String> {
    const C: &'static [u8] = b"0123456789ABCDEF";

    let message = nonce.to_owned() + customer_id + api_key;
    let mut hmac = Hmac::new(Sha256::new(), api_secret.as_bytes());

    hmac.input(message.as_bytes());
    let result = hmac.result();

    let raw_signature = result.code();
    let mut signature = Vec::with_capacity(raw_signature.len() * 2);
    for &byte in raw_signature {
        signature.push(C[(byte >> 4) as usize]);
        signature.push(C[(byte & 0xf) as usize]);
    }
    // TODO: Handle correctly the from_utf8 errors with error_chain.
    Ok(String::from_utf8(signature)?)
}

pub fn build_url(method: &str, pair: &str) -> String {
    "https://www.bitstamp.net/api/v2/".to_string() + method + "/" + pair + "/"
}

pub fn deserialize_json(json_string: &str) -> Result<Map<String, Value>> {
    let data: Value = match serde_json::from_str(json_string) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    match data.as_object() {
        Some(value) => Ok(value.clone()),
        None => Err(ErrorKind::BadParse.into()),
    }
}

pub fn generate_nonce(fixed_nonce: Option<String>) -> String {
    match fixed_nonce {
        Some(v) => v,
        None => helpers::get_unix_timestamp_ms().to_string(),
    }
}
