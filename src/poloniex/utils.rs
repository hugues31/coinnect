use bidir_map::BidirMap;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use crate::error::*;
use crate::types::Currency;
use crate::types::Pair;
use crate::types::Pair::*;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(BCN_BTC, "BTC_BCN");
        m.insert(BELA_BTC, "BTC_BELA");
        m.insert(BLK_BTC, "BTC_BLK");
        m.insert(BTCD_BTC, "BTC_BTCD");
        m.insert(BTM_BTC, "BTC_BTM");
        m.insert(BTS_BTC, "BTC_BTS");
        m.insert(BURST_BTC, "BTC_BURST");
        m.insert(CLAM_BTC, "BTC_CLAM");
        m.insert(DASH_BTC, "BTC_DASH");
        m.insert(DGB_BTC, "BTC_DGB");
        m.insert(DOGE_BTC, "BTC_DOGE");
        m.insert(EMC2_BTC, "BTC_EMC2");
        m.insert(FLDC_BTC, "BTC_FLDC");
        m.insert(FLO_BTC, "BTC_FLO");
        m.insert(GAME_BTC, "BTC_GAME");
        m.insert(GRC_BTC, "BTC_GRC");
        m.insert(HUC_BTC, "BTC_HUC");
        m.insert(LTC_BTC, "BTC_LTC");
        m.insert(MAID_BTC, "BTC_MAID");
        m.insert(OMNI_BTC, "BTC_OMNI");
        m.insert(NAUT_BTC, "BTC_NAUT");
        m.insert(NAV_BTC, "BTC_NAV");
        m.insert(NEOS_BTC, "BTC_NEOS");
        m.insert(NMC_BTC, "BTC_NMC");
        m.insert(NOTE_BTC, "BTC_NOTE");
        m.insert(NXT_BTC, "BTC_NXT");
        m.insert(PINK_BTC, "BTC_PINK");
        m.insert(POT_BTC, "BTC_POT");
        m.insert(PPC_BTC, "BTC_PPC");
        m.insert(RIC_BTC, "BTC_RIC");
        m.insert(SJCX_BTC, "BTC_SJCX");
        m.insert(STR_BTC, "BTC_STR");
        m.insert(SYS_BTC, "BTC_SYS");
        m.insert(VIA_BTC, "BTC_VIA");
        m.insert(XVC_BTC, "BTC_XVC");
        m.insert(VRC_BTC, "BTC_VRC");
        m.insert(VTC_BTC, "BTC_VTC");
        m.insert(XBC_BTC, "BTC_XBC");
        m.insert(XCP_BTC, "BTC_XCP");
        m.insert(XEM_BTC, "BTC_XEM");
        m.insert(XMR_BTC, "BTC_XMR");
        m.insert(XPM_BTC, "BTC_XPM");
        m.insert(XRP_BTC, "BTC_XRP");
        m.insert(BTC_USDT, "USDT_BTC");
        m.insert(DASH_USDT, "USDT_DASH");
        m.insert(LTC_USDT, "USDT_LTC");
        m.insert(NXT_USDT, "USDT_NXT");
        m.insert(STR_USDT, "USDT_STR");
        m.insert(XMR_USDT, "USDT_XMR");
        m.insert(XRP_USDT, "USDT_XRP");
        m.insert(BCN_XMR, "XMR_BCN");
        m.insert(BLK_XMR, "XMR_BLK");
        m.insert(BTCD_XMR, "XMR_BTCD");
        m.insert(DASH_XMR, "XMR_DASH");
        m.insert(LTC_XMR, "XMR_LTC");
        m.insert(MAID_XMR, "XMR_MAID");
        m.insert(NXT_XMR, "XMR_NXT");
        m.insert(ETH_BTC, "BTC_ETH");
        m.insert(ETH_USDT, "USDT_ETH");
        m.insert(SC_BTC, "BTC_SC");
        m.insert(BCY_BTC, "BTC_BCY");
        m.insert(EXP_BTC, "BTC_EXP");
        m.insert(FCT_BTC, "BTC_FCT");
        m.insert(RADS_BTC, "BTC_RADS");
        m.insert(AMP_BTC, "BTC_AMP");
        m.insert(DCR_BTC, "BTC_DCR");
        m.insert(LSK_BTC, "BTC_LSK");
        m.insert(LSK_ETH, "ETH_LSK");
        m.insert(LBC_BTC, "BTC_LBC");
        m.insert(STEEM_BTC, "BTC_STEEM");
        m.insert(STEEM_ETH, "ETH_STEEM");
        m.insert(SBD_BTC, "BTC_SBD");
        m.insert(ETC_BTC, "BTC_ETC");
        m.insert(ETC_ETH, "ETH_ETC");
        m.insert(ETC_USDT, "USDT_ETC");
        m.insert(REP_BTC, "BTC_REP");
        m.insert(REP_USDT, "USDT_REP");
        m.insert(REP_ETH, "ETH_REP");
        m.insert(ARDR_BTC, "BTC_ARDR");
        m.insert(ZEC_BTC, "BTC_ZEC");
        m.insert(ZEC_ETH, "ETH_ZEC");
        m.insert(ZEC_USDT, "USDT_ZEC");
        m.insert(ZEC_XMR, "XMR_ZEC");
        m.insert(STRAT_BTC, "BTC_STRAT");
        m.insert(NXC_BTC, "BTC_NXC");
        m.insert(PASC_BTC, "BTC_PASC");
        m.insert(GNT_BTC, "BTC_GNT");
        m.insert(GNT_ETH, "ETH_GNT");
        m.insert(GNO_BTC, "BTC_GNO");
        m.insert(GNO_ETH, "ETH_GNO");
        m.insert(BCH_BTC, "BTC_BCH");
        m.insert(BCH_ETH, "ETH_BCH");
        m.insert(BCH_USDT, "USDT_BCH");
        m.insert(ZRX_BTC, "BTC_ZRX");
        m.insert(ZRX_ETH, "ETH_ZRX");
        m.insert(CVC_BTC, "BTC_CVC");
        m.insert(CVC_ETH, "ETH_CVC");
        m.insert(OMG_BTC, "BTC_OMG");
        m.insert(OMG_ETH, "ETH_OMG");
        m.insert(GAS_BTC, "BTC_GAS");
        m.insert(GAS_ETH, "ETH_GAS");
        m.insert(STORJ_BTC, "BTC_STORJ");
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

/// Convert a JSON array into a map containing a Vec for the "data" key
pub fn deserialize_json_array(json_string: &str) -> Result<Map<String, Value>> {
    let data: Value = match serde_json::from_str(json_string) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    if data.is_array() {
        let mut map = Map::new();
        map.insert("data".to_string(), data);
        Ok(map)
    }

    else {
        Err(ErrorKind::BadParse.into())
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
/// string used by Poloniex. If no currency is found,
/// return None
/// # Examples
///
/// ```
/// use crate::coinnect::poloniex::utils::get_currency_enum;
/// use crate::coinnect::types::Currency;
///
/// let currency = get_currency_enum("BTC").unwrap();
/// assert_eq!(currency, Currency::BTC);
/// ```
pub fn get_currency_enum(currency: &str) -> Option<Currency> {
    match currency {
        "AMP" => Some(Currency::AMP),
        "BTC" => Some(Currency::BTC),
        "ARDR" => Some(Currency::ARDR),
        "ETH" => Some(Currency::ETH),
        "ETC" => Some(Currency::ETC),
        "LBC" => Some(Currency::LBC),
        "XMR" => Some(Currency::XMR),
        "XPM" => Some(Currency::XPM),
        "XRP" => Some(Currency::XRP),
        "XVC" => Some(Currency::XVC),
        "ZEC" => Some(Currency::ZEC),
        _ => None,
    }
}

/// Return the currency string associated with the
/// enum used by Poloniex. If no currency is found,
/// return None
/// # Examples
///
/// ```
/// use crate::coinnect::poloniex::utils::get_currency_string;
/// use crate::coinnect::types::Currency;
///
/// let currency = get_currency_string(Currency::BTC);
/// assert_eq!(currency, Some("BTC".to_string()));
/// ```
pub fn get_currency_string(currency: Currency) -> Option<String> {
    match currency {
        Currency::AMP => Some("AMP".to_string()),
        Currency::BTC => Some("BTC".to_string()),
        Currency::ARDR => Some("ARDR".to_string()),
        Currency::ETH => Some("ETH".to_string()),
        Currency::ETC => Some("ETC".to_string()),
        Currency::LBC => Some("LBC".to_string()),
        Currency::XMR => Some("XMR".to_string()),
        Currency::XPM => Some("XPM".to_string()),
        Currency::XRP => Some("XRP".to_string()),
        Currency::XVC => Some("XVC".to_string()),
        Currency::ZEC => Some("ZEC".to_string()),
        _ => None,
    }
}
