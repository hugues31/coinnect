//! This module contains Pair enum.

// Order of quote currency <-> base currency is important. For example, Kraken supports ZEC_BTC
// but Poloniex is doing the opposite :  BTC_ZEC, which equal to 1/ZEC_BTC. So: ZEC_BTC != BTC_ZEC

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Pair {
    BTC_EUR,
    BTC_USD,
    BTC_ZEC,
    BTC_ETC,
    BTC_ETH,
    BTC_XMR,
    ETC_BTC,
    ETC_ETH,
    ETC_EUR,
    ETC_USD,
    ETH_BTC,
    ETH_EUR,
    ETH_USD,
    EUR_USD,
    DASH_BTC,
    ICN_BTC,
    ICN_ETH,
    LTC_BTC,
    LTC_EUR,
    LTC_USD,
    XMR_BTC,
    XMR_EUR,
    XMR_USD,
    XRP_BTC,
    XRP_EUR,
    XRP_USD,
    ZEC_BTC,
    ZEC_EUR,
    ZEC_USD,
}
