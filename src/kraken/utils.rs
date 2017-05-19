use bidir_map::BidirMap;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error;
use pair::Pair;
use pair::Pair::*;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(BTC_CAD_d, "XXBTZCAD.d");
        m.insert(BTC_CAD,   "XXBTZCAD");
        m.insert(BTC_EUR_d, "XXBTZEUR.d");
        m.insert(BTC_EUR,   "XXBTZEUR");
        m.insert(BTC_GBP_d, "XXBTZGBP.d");
        m.insert(BTC_GBP,   "XXBTZGBP");
        m.insert(BTC_JPY_d, "XXBTZJPY.d");
        m.insert(BTC_JPY,   "XXBTZJPY");
        m.insert(BTC_USD_d, "XXBTZUSD.d");
        m.insert(BTC_USD,   "XXBTZUSD");
        m.insert(ETC_BTC,   "XETCXXBT");
        m.insert(ETC_ETH,   "XETCXETH");
        m.insert(ETC_EUR,   "XETCZEUR");
        m.insert(ETC_USD,   "XETCZUSD");
        m.insert(ETH_BTC,   "XETHXXBT");
        m.insert(ETH_CAD_d, "XETHZCAD.d");
        m.insert(ETH_CAD,   "XETHZCAD");
        m.insert(ETH_EUR_d, "XETHZEUR.d");
        m.insert(ETH_EUR,   "XETHZEUR");
        m.insert(ETH_GBP_d, "XETHZGBP.d");
        m.insert(ETH_GBP,   "XETHZGBP");
        m.insert(ETH_JPY_d, "XETHZJPY.d");
        m.insert(ETH_JPY,   "XETHZJPY");
        m.insert(ETH_USD_d, "XETHZUSD.d");
        m.insert(ETH_USD,   "XETHZUSD");
        m.insert(ETH_XBT_d, "XETHXXBT.d");
        m.insert(GNO_BTC,   "GNOXBT");
        m.insert(GNO_ETH,   "GNOETH");
        m.insert(GNO_EUR,   "GNOEUR");
        m.insert(GNO_USD,   "GNOUSD");
        m.insert(ICN_BTC,   "XICNXXBT");
        m.insert(ICN_ETH,   "XICNXETH");
        m.insert(LTC_BTC,   "XLTCXXBT");
        m.insert(LTC_EUR,   "XLTCZEUR");
        m.insert(LTC_USD,   "XLTCZUSD");
        m.insert(MLN_BTC,   "XMLNXXBT");
        m.insert(MLN_ETH,   "XMLNXETH");
        m.insert(REP_BTC,   "XREPXXBT");
        m.insert(REP_ETH,   "XREPXETH");
        m.insert(REP_EUR,   "XREPZEUR");
        m.insert(REP_USD,   "XREPZUSD");
        m.insert(USDT_USD,  "USDTZUSD");
        m.insert(XDG_BTC,   "XXDGXXBT");
        m.insert(XLM_BTC,   "XXLMXXBT");
        m.insert(XLM_EUR,   "XXLMZEUR");
        m.insert(XLM_USD,   "XXLMZUSD");
        m.insert(XMR_BTC,   "XXMRXXBT");
        m.insert(XMR_EUR,   "XXMRZEUR");
        m.insert(XMR_USD,   "XXMRZUSD");
        m.insert(XRP_BTC,   "XXRPXXBT");
        m.insert(ZEC_BTC,   "XZECXXBT");
        m.insert(ZEC_EUR,   "XZECZEUR");
        m.insert(ZEC_USD,   "XZECZUSD");
        m
    };
}

/// Return the name associated to pair used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: &Pair) -> Option<&&str> {
    PAIRS_STRING.get_by_first(pair)
}

/// Return the Pair enum associated to the string used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_enum(pair: &str) -> Option<&Pair> {
    PAIRS_STRING.get_by_second(&pair)
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
        "EOrder:Insufficient funds" => Err(error::Error::InsufficientFunds),
        "EOrder:Order minimum not met" => Err(error::Error::InsufficientOrderSize),
        other => Err(error::Error::ExchangeSpecificError(other.to_string())),
    }
}
