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
        m.insert(BTC_AMP, "BTC_AMP");
        m.insert(BTC_ARDR, "BTC_ARDR");
        m.insert(BTC_BCN, "BTC_BCN");
        m.insert(BTC_BCY, "BTC_BCY");
        m.insert(BTC_BELA, "BTC_BELA");
        m.insert(BTC_BLK, "BTC_BLK");
        m.insert(BTC_BTCD, "BTC_BTCD");
        m.insert(BTC_BTM, "BTC_BTM");
        m.insert(BTC_BTS, "BTC_BTS");
        m.insert(BTC_BURST, "BTC_BURST");
        m.insert(BTC_CLAM, "BTC_CLAM");
        m.insert(BTC_DASH, "BTC_DASH");
        m.insert(BTC_DCR, "BTC_DCR");
        m.insert(BTC_DGB, "BTC_DGB");
        m.insert(BTC_DOGE, "BTC_DOGE");
        m.insert(BTC_EMC2, "BTC_EMC2");
        m.insert(BTC_ETC, "BTC_ETC");
        m.insert(BTC_ETH, "BTC_ETH");
        m.insert(BTC_EXP, "BTC_EXP");
        m.insert(BTC_FCT, "BTC_FCT");
        m.insert(BTC_FLDC, "BTC_FLDC");
        m.insert(BTC_FLO, "BTC_FLO");
        m.insert(BTC_GAME, "BTC_GAME");
        m.insert(BTC_GNO, "BTC_GNO");
        m.insert(BTC_GNT, "BTC_GNT");
        m.insert(BTC_GRC, "BTC_GRC");
        m.insert(BTC_HUC, "BTC_HUC");
        m.insert(BTC_LBC, "BTC_LBC");
        m.insert(BTC_LSK, "BTC_LSK");
        m.insert(BTC_LTC, "BTC_LTC");
        m.insert(BTC_MAID, "BTC_MAID");
        m.insert(BTC_NAUT, "BTC_NAUT");
        m.insert(BTC_NAV, "BTC_NAV");
        m.insert(BTC_NEOS, "BTC_NEOS");
        m.insert(BTC_NMC, "BTC_NMC");
        m.insert(BTC_NOTE, "BTC_NOTE");
        m.insert(BTC_NXC, "BTC_NXC");
        m.insert(BTC_NXT, "BTC_NXT");
        m.insert(BTC_OMNI, "BTC_OMNI");
        m.insert(BTC_PASC, "BTC_PASC");
        m.insert(BTC_PINK, "BTC_PINK");
        m.insert(BTC_POT, "BTC_POT");
        m.insert(BTC_PPC, "BTC_PPC");
        m.insert(BTC_RADS, "BTC_RADS");
        m.insert(BTC_REP, "BTC_REP");
        m.insert(BTC_RIC, "BTC_RIC");
        m.insert(BTC_SBD, "BTC_SBD");
        m.insert(BTC_SC, "BTC_SC");
        m.insert(BTC_SJCX, "BTC_SJCX");
        m.insert(BTC_STEEM, "BTC_STEEM");
        m.insert(BTC_STR, "BTC_STR");
        m.insert(BTC_STRAT, "BTC_STRAT");
        m.insert(BTC_SYS, "BTC_SYS");
        m.insert(BTC_VIA, "BTC_VIA");
        m.insert(BTC_VRC, "BTC_VRC");
        m.insert(BTC_VTC, "BTC_VTC");
        m.insert(BTC_XBC, "BTC_XBC");
        m.insert(BTC_XCP, "BTC_XCP");
        m.insert(BTC_XEM, "BTC_XEM");
        m.insert(BTC_XMR, "BTC_XMR");
        m.insert(BTC_XPM, "BTC_XPM");
        m.insert(BTC_XRP, "BTC_XRP");
        m.insert(BTC_XVC, "BTC_XVC");
        m.insert(BTC_ZEC, "BTC_ZEC");
        m.insert(ETH_ETC, "ETH_ETC");
        m.insert(ETH_GNO, "ETH_GNO");
        m.insert(ETH_GNT, "ETH_GNT");
        m.insert(ETH_LSK, "ETH_LSK");
        m.insert(ETH_REP, "ETH_REP");
        m.insert(ETH_STEEM, "ETH_STEEM");
        m.insert(ETH_ZEC, "ETH_ZEC");
        m.insert(USDT_BTC, "USDT_BTC");
        m.insert(USDT_DASH, "USDT_DASH");
        m.insert(USDT_ETC, "USDT_ETC");
        m.insert(USDT_ETH, "USDT_ETH");
        m.insert(USDT_LTC, "USDT_LTC");
        m.insert(USDT_NXT, "USDT_NXT");
        m.insert(USDT_REP, "USDT_REP");
        m.insert(USDT_STR, "USDT_STR");
        m.insert(USDT_XMR, "USDT_XMR");
        m.insert(USDT_XRP, "USDT_XRP");
        m.insert(USDT_ZEC, "USDT_ZEC");
        m.insert(XMR_BCN, "XMR_BCN");
        m.insert(XMR_BLK, "XMR_BLK");
        m.insert(XMR_BTCD, "XMR_BTCD");
        m.insert(XMR_DASH, "XMR_DASH");
        m.insert(XMR_LTC, "XMR_LTC");
        m.insert(XMR_MAID, "XMR_MAID");
        m.insert(XMR_NXT, "XMR_NXT");
        m.insert(XMR_ZEC, "XMR_ZEC");

        m
    };
}

/// Return the name associated to pair used by Poloniex
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: &Pair) -> Option<&&str> {
    PAIRS_STRING.get_by_first(pair)
}

/// Return the Pair enum associated to the string used by Poloniex
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
    let error_msg = match response.get("error") {
        Some(error) => error.as_str().unwrap(),
        None => return Ok(response.clone()),
    };

    match error_msg.as_ref() {
        "Invalid command." => Err(error::Error::InvalidArguments),
        other => Err(error::Error::ExchangeSpecificError(other.to_string())),
    }
}
