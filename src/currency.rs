//! This module contains Curreny enum.

/// Currency lists all currencies that can be traded on supported exchanges.
/// Update date : 26/05/2017.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Currency {
    AMP,
    ARDR,
    BCN,
    BCY,
    BELA,
    BLK,
    BTC,
    BTCD,
    BTM,
    BTS,
    BURST,
    CAD,
    CLAM,
    DASH,
    DCR,
    DGB,
    DOGE,
    EMC2,
    ETC,
    ETH,
    EUR,
    EXP,
    FCT,
    FLDC,
    FLO,
    GAME,
    GBP,
    GNO,
    GNT,
    GRC,
    HUC,
    ICN,
    JPY,
    LBC,
    LSK,
    LTC,
    MAID,
    MLN,
    NAUT,
    NAV,
    NEOS,
    NMC,
    NOTE,
    NXC,
    NXT,
    OMNI,
    PASC,
    PINK,
    POT,
    PPC,
    RADS,
    REP,
    RIC,
    SBD,
    SC,
    SJCX,
    STEEM,
    STR,
    STRAT,
    SYS,
    USD,
    USDT,
    VIA,
    VRC,
    VTC,
    XBC,
    XCP,
    XDG,
    XEM,
    XLM,
    XMR,
    XPM,
    XRP,
    XVC,
    ZEC,
}


pub trait Operations {
    fn to_string(&self) -> String;
    fn from_str(&str) -> Option<Currency>;
}

impl Operations for Currency {

    fn from_str(currency: &str) -> Option<Currency> {
        match currency {
            "AMP" => Some(Currency::AMP),
            "ARDR" => Some(Currency::ARDR),
            "BCN" => Some(Currency::BCN),
            "BCY" => Some(Currency::BCY),
            "BELA" => Some(Currency::BELA),
            "BLK" => Some(Currency::BLK),
            "BTC" => Some(Currency::BTC),
            "BTCD" => Some(Currency::BTCD),
            "BTM" => Some(Currency::BTM),
            "BTS" => Some(Currency::BTS),
            "BURST" => Some(Currency::BURST),
            "CAD" => Some(Currency::CAD),
            "CLAM" => Some(Currency::CLAM),
            "DASH" => Some(Currency::DASH),
            "DCR" => Some(Currency::DCR),
            "DGB" => Some(Currency::DGB),
            "DOGE" => Some(Currency::DOGE),
            "EMC2" => Some(Currency::EMC2),
            "ETC" => Some(Currency::ETC),
            "ETH" => Some(Currency::ETH),
            "EUR" => Some(Currency::EUR),
            "EXP" => Some(Currency::EXP),
            "FCT" => Some(Currency::FCT),
            "FLDC" => Some(Currency::FLDC),
            "FLO" => Some(Currency::FLO),
            "GAME" => Some(Currency::GAME),
            "GBP" => Some(Currency::GBP),
            "GNO" => Some(Currency::GNO),
            "GNT" => Some(Currency::GNT),
            "GRC" => Some(Currency::GRC),
            "HUC" => Some(Currency::HUC),
            "ICN" => Some(Currency::ICN),
            "JPY" => Some(Currency::JPY),
            "LBC" => Some(Currency::LBC),
            "LSK" => Some(Currency::LSK),
            "LTC" => Some(Currency::LTC),
            "MAID" => Some(Currency::MAID),
            "MLN" => Some(Currency::MLN),
            "NAUT" => Some(Currency::NAUT),
            "NAV" => Some(Currency::NAV),
            "NEOS" => Some(Currency::NEOS),
            "NMC" => Some(Currency::NMC),
            "NOTE" => Some(Currency::NOTE),
            "NXC" => Some(Currency::NXC),
            "NXT" => Some(Currency::NXT),
            "OMNI" => Some(Currency::OMNI),
            "PASC" => Some(Currency::PASC),
            "PINK" => Some(Currency::PINK),
            "POT" => Some(Currency::POT),
            "PPC" => Some(Currency::PPC),
            "RADS" => Some(Currency::RADS),
            "REP" => Some(Currency::REP),
            "RIC" => Some(Currency::RIC),
            "SBD" => Some(Currency::SBD),
            "SC" => Some(Currency::SC),
            "SJCX" => Some(Currency::SJCX),
            "STEEM" => Some(Currency::STEEM),
            "STR" => Some(Currency::STR),
            "STRAT" => Some(Currency::STRAT),
            "SYS" => Some(Currency::SYS),
            "USD" => Some(Currency::USD),
            "USDT" => Some(Currency::USDT),
            "VIA" => Some(Currency::VIA),
            "VRC" => Some(Currency::VRC),
            "VTC" => Some(Currency::VTC),
            "XBC" => Some(Currency::XBC),
            "XCP" => Some(Currency::XCP),
            "XDG" => Some(Currency::XDG),
            "XEM" => Some(Currency::XEM),
            "XLM" => Some(Currency::XLM),
            "XMR" => Some(Currency::XMR),
            "XPM" => Some(Currency::XPM),
            "XRP" => Some(Currency::XRP),
            "XVC" => Some(Currency::XVC),
            "ZEC" => Some(Currency::ZEC),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_currency() {
        assert_eq!(Currency::BTC.to_string(), "BTC".to_string());
        assert_eq!(Currency::XMR.to_string(), "XMR".to_string());
        assert_eq!(Currency::USD.to_string(), "USD".to_string());
        assert_eq!(Currency::USDT.to_string(), "USDT".to_string());

        assert_eq!(Currency::from_str("BTC"), Some(Currency::BTC));
        assert_eq!(Currency::from_str("USD"), Some(Currency::USD));
        assert_eq!(Currency::from_str("XxX"), None);
    }
}
