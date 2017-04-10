use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error;
use pair::Pair;
use pair::Pair::*;

/// Return the name associated to pair used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: &Pair) -> Option<String> {
    let pair_string = match *pair {
        BTC_EUR => "XXBTZEUR".to_string(),
        BTC_USD => "XXBTZUSD".to_string(),
        ETC_BTC => "XETCXXBT".to_string(),
        ETC_ETH => "XETCXETH".to_string(),
        ETC_EUR => "XETCZEUR".to_string(),
        ETC_USD => "XETCZUSD".to_string(),
        ETH_BTC => "XETHXXBT".to_string(),
        ETH_EUR => "XETHZEUR".to_string(),
        ETH_USD => "XETHZUSD".to_string(),
        ICN_BTC => "XICNXXBT".to_string(),
        ICN_ETH => "XICNXETH".to_string(),
        LTC_BTC => "XLTCXXBT".to_string(),
        LTC_EUR => "XLTCZEUR".to_string(),
        LTC_USD => "XLTCZUSD".to_string(),
        XMR_BTC => "XXMRXBTC".to_string(),
        XMR_EUR => "XXMRZEUR".to_string(),
        XMR_USD => "XXMRZUSD".to_string(),
        XRP_BTC => "XXRPXBTC".to_string(),
        XRP_EUR => "XXRPZEUR".to_string(),
        XRP_USD => "XXRPZUSD".to_string(),
        ZEC_BTC => "XZECXBTC".to_string(),
        ZEC_EUR => "XZECZEUR".to_string(),
        ZEC_USD => "XZECZUSD".to_string(),
        _ => return None,
    };
    Some(pair_string)
}

/// Return the name associated to pair used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_enum(pair: String) -> Option<Pair> {
    let pair_option = match pair.as_ref() {
        "BTC_EUR" => BTC_EUR,
        "BTC_USD" => BTC_USD,
        "ETC_BTC" => ETC_BTC,
        "ETC_ETH" => ETC_ETH,
        "ETC_EUR" => ETC_EUR,
        "ETC_USD" => ETC_USD,
        "ETH_BTC" => ETH_BTC,
        "ETH_EUR" => ETH_EUR,
        "ETH_USD" => ETH_USD,
        "EUR_USD" => EUR_USD,
        "ICN_BTC" => ICN_BTC,
        "ICN_ETH" => ICN_ETH,
        "LTC_BTC" => LTC_BTC,
        "LTC_EUR" => LTC_EUR,
        "LTC_USD" => LTC_USD,
        "XMR_BTC" => XMR_BTC,
        "XMR_EUR" => XMR_EUR,
        "XMR_USD" => XMR_USD,
        "XRP_BTC" => XRP_BTC,
        "XRP_EUR" => XRP_EUR,
        "XRP_USD" => XRP_USD,
        "ZEC_BTC" => ZEC_BTC,
        "ZEC_EUR" => ZEC_EUR,
        "ZEC_USD" => ZEC_USD,
        _ => return None,
    };
    Some(pair_option)
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

/// If error array is null, return the result (encoded in a json object)
/// else return the error string found in array
pub fn parse_result(response: Map<String, Value>) -> Result<Map<String, Value>, error::Error> {
    let error_array = match response.get("error") {
        Some(array) => array.as_array().unwrap(),
        None => return Err(error::Error::BadParse),
    };
    if error_array.is_empty() {
        return Ok(response.get("result").unwrap().as_object().unwrap().clone());
    }
    let error_msg = error_array[0].as_str().unwrap().to_string();

    match error_msg.as_ref() {
        "EService:Unavailable" => Err(error::Error::ServiceUnavailable),
        "EOrder:Rate limit exceeded" => Err(error::Error::RateLimitExceeded),
        "EQuery:Unknown asset pair" => Err(error::Error::PairUnsupported),
        "EGeneral:Invalid arguments" => Err(error::Error::InvalidArguments),
        other => Err(error::Error::ExchangeSpecificError(other.to_string())),
    }
}
