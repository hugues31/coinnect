use bidir_map::BidirMap;

use hmac::{Hmac, Mac};
use sha2::{Sha256};

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use crate::error::*;
use crate::helpers;
use crate::types::Currency;
use crate::types::Pair;
use crate::types::Pair::*;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(BTC_USD, "btcusd");
        m.insert(BTC_EUR, "btceur");
        m.insert(EUR_USD, "eurusd");
        m.insert(XRP_USD, "xrpusd");
        m.insert(XRP_EUR, "xrpeur");
        m.insert(XRP_BTC, "xrpbtc");
        m.insert(LTC_USD, "ltcusd");
        m.insert(LTC_EUR, "ltceur");
        m.insert(LTC_BTC, "ltcbtc");
        m.insert(ETH_USD, "ethusd");
        m.insert(ETH_EUR, "etheur");
        m.insert(ETH_BTC, "ethbtc");
        m.insert(BCH_USD, "bchusd");
        m.insert(BCH_EUR, "bcheur");
        m.insert(BCH_BTC, "bchbtc");
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

pub fn build_signature(nonce: &str,
                       customer_id: &str,
                       api_key: &str,
                       api_secret: &str)
                       -> Result<String> {
    const C: &'static [u8] = b"0123456789ABCDEF";

    let message = nonce.to_owned() + customer_id + api_key;

    let mut mac = Hmac::<Sha256>::new(api_secret.as_bytes());

    mac.input(message.as_bytes());
    let result = mac.result();

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

/// If error array is null, return the result (encoded in a json object)
/// else return the error string found in array
pub fn parse_result(response: &Map<String, Value>) -> Result<Map<String, Value>> {
    let error_msg = match response.get("error") {
        Some(error) => {
            error
                .as_str()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat("error".to_string()))?
        }
        None => return Ok(response.clone()),
    };

    match error_msg.as_ref() {
        "Invalid command." => Err(ErrorKind::InvalidArguments.into()),
        "Invalid API key/secret pair." => Err(ErrorKind::BadCredentials.into()),
        "Total must be at least 0.0001." => Err(ErrorKind::InsufficientOrderSize.into()),
        other => Err(ErrorKind::ExchangeSpecificError(other.to_string()).into()),
    }
}

/// Return the currency enum associated with the
/// string used by Bitstamp. If no currency is found,
/// return None
/// # Examples
///
/// ```
/// use crate::coinnect::bitstamp::utils::get_currency_enum;
/// use crate::coinnect::types::Currency;
///
/// let currency = get_currency_enum("usd_balance");
/// assert_eq!(Some(Currency::USD), currency);
/// ```
pub fn get_currency_enum(currency: &str) -> Option<Currency> {
    match currency {
        "usd_balance" => Some(Currency::USD),
        "btc_balance" => Some(Currency::BTC),
        "eur_balance" => Some(Currency::EUR),
        "xrp_balance" => Some(Currency::XRP),
        _ => None,
    }
}

/// Return the currency string associated with the
/// enum used by Bitstamp. If no currency is found,
/// return None
/// # Examples
///
/// ```
/// use crate::coinnect::bitstamp::utils::get_currency_string;
/// use crate::coinnect::types::Currency;
///
/// let currency = get_currency_string(Currency::USD);
/// assert_eq!(currency, Some("USD".to_string()));
/// ```
pub fn get_currency_string(currency: Currency) -> Option<String> {
    match currency {
        Currency::USD => Some("USD".to_string()),
        Currency::BTC => Some("BTC".to_string()),
        Currency::EUR => Some("EUR".to_string()),
        Currency::XRP => Some("XRP".to_string()),
        _ => None,
    }
}
