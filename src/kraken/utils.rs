use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error;
use pair::Pair;
use pair::Pair::*;

/// Return the name associated to pair used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: Pair) -> Option<String> {
    let pair_string = match pair {
        BTC_EUR => "XXBTZEUR".to_string(),
        BTC_USD => "XXBTZUSD".to_string(),
        ETC_BTC => "XETCXXBT".to_string(),
        ETC_ETH => "XETCXETH".to_string(),
        ETC_EUR => "XETCZEUR".to_string(),
        ETC_USD => "XETCZUSD".to_string(),
        ETH_BTC => "XETHXXBT".to_string(),
        ETH_EUR => "XETHZEUR".to_string(),
        ETH_USD => "XETHZUSD".to_string(),
        EUR_USD => return None,
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
    let pair_option = match pair {
        String::new("BTC_EUR") => BTC_EUR,
        "BTC_USD".to_string() => BTC_USD,
        "ETC_BTC".to_string() => ETC_BTC,
        "ETC_ETH".to_string() => ETC_ETH,
        "ETC_EUR".to_string() => ETC_EUR,
        "ETC_USD".to_string() => ETC_USD,
        "ETH_BTC".to_string() => ETH_BTC,
        "ETH_EUR".to_string() => ETH_EUR,
        "ETH_USD".to_string() => ETH_USD,
        "EUR_USD".to_string() => EUR_USD,
        "ICN_BTC".to_string() => ICN_BTC,
        "ICN_ETH".to_string() => ICN_ETH,
        "LTC_BTC".to_string() => LTC_BTC,
        "LTC_EUR".to_string() => LTC_EUR,
        "LTC_USD".to_string() => LTC_USD,
        "XMR_BTC".to_string() => XMR_BTC,
        "XMR_EUR".to_string() => XMR_EUR,
        "XMR_USD".to_string() => XMR_USD,
        "XRP_BTC".to_string() => XRP_BTC,
        "XRP_EUR".to_string() => XRP_EUR,
        "XRP_USD".to_string() => XRP_USD,
        "ZEC_BTC".to_string() => ZEC_BTC,
        "ZEC_EUR".to_string() => ZEC_EUR,
        "ZEC_USD".to_string() => ZEC_USD,
        _ => return None,
    };
    Some(pair_option)
}

pub fn deserialize_json(json_string: String) -> Result<Map<String, Value>, error::Error> {
    let data: Value = match serde_json::from_str(&json_string) {
        Ok(data) => data,
        Err(_) => return Err(error::Error::BadParse)
    };

    match data.as_object() {
        Some(value) => Ok(value.clone()),
        None => Err(error::Error::BadParse),
    }
}
