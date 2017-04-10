use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::thread;
use std::time::Duration;

use error;
use helpers;

pub fn block_or_continue(last_request: i64) {
    let threshold = 1000; // 600 requests per 10 mins = 1 request per second
    let delay = helpers::get_unix_timestamp_ms() - last_request;
    if delay < threshold {
        let duration_ms = Duration::from_millis(delay as u64);
        thread::sleep(duration_ms);
    }
}

pub fn build_signature(nonce: String,
                       customer_id: String,
                       api_key: String,
                       api_secret: String)
                       -> String {
    const C: &'static [u8] = b"0123456789ABCDEF";

    let message = nonce + &customer_id + &api_key;
    let mut hmac = Hmac::new(Sha256::new(), api_secret.as_bytes());

    hmac.input(message.as_bytes());
    let result = hmac.result();

    let raw_signature = result.code();
    let mut signature = Vec::with_capacity(raw_signature.len() * 2);
    for &byte in raw_signature {
        signature.push(C[(byte >> 4) as usize]);
        signature.push(C[(byte & 0xf) as usize]);
    }
    String::from_utf8(signature).unwrap()
}

pub fn build_url(method: &str, pair: &str) -> String {
    "https://www.bitstamp.net/api/v2/".to_string() + method + "/" + &pair + "/"
}

pub fn deserialize_json(json_string: String) -> Result<Map<String, Value>, error::Error> {
    let data: Value = match serde_json::from_str(&json_string) {
        Ok(data) => data,
        Err(_) => return Err(error::Error::BadParse),
    };

    match data.as_object() {
        Some(value) => Ok(value.clone()),
        None => Err(error::Error::BadParse),
    }
}

pub fn generate_nonce(fixed_nonce: Option<String>) -> String {
    match fixed_nonce {
        Some(v) => v,
        None => helpers::get_unix_timestamp_ms().to_string(),
    }
}
