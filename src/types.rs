//! Types definition used for handling returned data when generic API is used.

use std::collections::HashMap;
use bigdecimal::BigDecimal;
use std::str::FromStr;


pub type Amount = BigDecimal;
pub type Price = BigDecimal;
pub type Volume = BigDecimal;

pub type Balances = HashMap<Currency, Amount>;

#[derive(Debug)]
pub struct Ticker {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// The Pair corresponding to the Ticker returned (maybe useful later for asynchronous APIs)
    pub pair: Pair,
    /// Last trade price found in the history
    pub last_trade_price: Price,
    /// Lowest ask price found in Orderbook
    pub lowest_ask: Price,
    /// Highest bid price found in Orderbook
    pub highest_bid: Price,
    // Bittrex does not support Volume for ticker so volume could be None
    /// Last 24 hours volume (quote-volume)
    pub volume: Option<Volume>,
}

#[derive(Debug)]
pub struct Orderbook {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// The Pair corresponding to the Orderbook returned (maybe useful later for asynchronous APIs)
    pub pair: Pair,
    /// Vec containing the ask offers (by ascending price)
    pub asks: Vec<(Price, Volume)>,
    /// Vec containing the bid offers (by descending price)
    pub bids: Vec<(Price, Volume)>,
}

impl Orderbook {
    /// Convenient function that returns the average price from the orderbook
    /// Return None if Orderbook is empty
    /// `Average price = (lowest ask + highest bid)/2`
    pub fn avg_price(&self) -> Option<Price> {
        if self.asks.is_empty() || self.bids.is_empty() {
            return None;
        }
        Some(
            (self.asks[0].0.clone() + self.bids[0].0.clone())
            /
            BigDecimal::from_str("2.0").unwrap()
        )
    }
}

#[derive(Debug)]
pub struct OrderInfo {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// This identifiers list is specific to the platform you use. You must store it somewhere if
    /// you want to modify/cancel the order later
    pub identifier: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum OrderType {
    BuyLimit,
    SellLimit,
    BuyMarket,
    SellMarket,
}

/// Currency lists all currencies that can be traded on supported exchanges.
/// Update date : 26/05/2017.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Currency {
    AMP,
    ARDR,
    BCH,
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
    EOS,
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

/// Pair lists all pairs that can be traded on supported exchanges.
/// Update date : 20/05/2017.
///
/// Order of quote currency <-> base currency is important. For example, Kraken supports ZEC_BTC
/// but Poloniex is doing the opposite inside their API: BTC_ZEC, which equal to 1/ZEC_BTC.
/// So: ZEC_BTC != BTC_ZEC but Poloniex ignores this and decided that BTC_ZEC = ZEC_BTC, so
/// that 1 ZEC = ZEC_BTC pair value. To standardize, the following pair uses the standard notation.
/// (so all Poloniex pair has been flipped)
///
/// Note : Kraken uses 'XBT' instead of 'BTC' (so the XBT/EUR pair becomes BTC/EUR).
///
/// To summarize, Kraken uses the pair 'ZEC_XBT', whereas Poloniex uses the 'BTC_ZEC' pair. With
/// the standardization proposed above these 2 pairs become 'ZEC_BTC', that are comparable in
/// value accross the 2 exchanges.
/// Pairs with "_d" at the end : dark pool
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Pair {
    AMP_BTC,
    ARDR_BTC,
    BCH_BTC,
    BCH_EUR,
    BCH_USD,
    BCN_BTC,
    BCN_XMR,
    BCY_BTC,
    BELA_BTC,
    BLK_BTC,
    BLK_XMR,
    BTCD_BTC,
    BTCD_XMR,
    BTC_CAD,
    BTC_CAD_d,
    BTC_EUR,
    BTC_EUR_d,
    BTC_GBP,
    BTC_GBP_d,
    BTC_JPY,
    BTC_JPY_d,
    BTC_USD,
    BTC_USDT,
    BTC_USD_d,
    BTM_BTC,
    BTS_BTC,
    BURST_BTC,
    CLAM_BTC,
    DASH_BTC,
    DASH_EUR,
    DASH_USD,
    DASH_USDT,
    DASH_XMR,
    DCR_BTC,
    DGB_BTC,
    DOGE_BTC,
    EMC2_BTC,
    EOS_BTC,
    EOS_ETH,
    EOS_EUR,
    EOS_USD,
    ETC_BTC,
    ETC_ETH,
    ETC_EUR,
    ETC_USD,
    ETC_USDT,
    ETH_BTC,
    ETH_BTC_d,
    ETH_CAD,
    ETH_CAD_d,
    ETH_EUR,
    ETH_EUR_d,
    ETH_GBP,
    ETH_GBP_d,
    ETH_JPY,
    ETH_JPY_d,
    ETH_USD,
    ETH_USDT,
    ETH_USD_d,
    EUR_USD,
    EXP_BTC,
    FCT_BTC,
    FLDC_BTC,
    FLO_BTC,
    GAME_BTC,
    GNO_BTC,
    GNO_ETH,
    GNO_EUR,
    GNO_USD,
    GNT_BTC,
    GNT_ETH,
    GRC_BTC,
    HUC_BTC,
    ICN_BTC,
    ICN_ETH,
    LBC_BTC,
    LSK_BTC,
    LSK_ETH,
    LTC_BTC,
    LTC_EUR,
    LTC_USD,
    LTC_USDT,
    LTC_XMR,
    MAID_BTC,
    MAID_XMR,
    MLN_BTC,
    MLN_ETH,
    NAUT_BTC,
    NAV_BTC,
    NEOS_BTC,
    NMC_BTC,
    NOTE_BTC,
    NXC_BTC,
    NXT_BTC,
    NXT_USDT,
    NXT_XMR,
    OMNI_BTC,
    PASC_BTC,
    PINK_BTC,
    POT_BTC,
    PPC_BTC,
    RADS_BTC,
    REP_BTC,
    REP_ETH,
    REP_EUR,
    REP_USD,
    REP_USDT,
    RIC_BTC,
    SBD_BTC,
    SC_BTC,
    SJCX_BTC,
    STEEM_BTC,
    STEEM_ETH,
    STRAT_BTC,
    STR_BTC,
    STR_USDT,
    SYS_BTC,
    USDT_USD,
    VIA_BTC,
    VRC_BTC,
    VTC_BTC,
    XBC_BTC,
    XCP_BTC,
    XDG_BTC,
    XEM_BTC,
    XLM_BTC,
    XLM_EUR,
    XLM_USD,
    XMR_BTC,
    XMR_EUR,
    XMR_USD,
    XMR_USDT,
    XPM_BTC,
    XRP_BTC,
    XRP_CAD,
    XRP_EUR,
    XRP_JPY,
    XRP_USD,
    XRP_USDT,
    XVC_BTC,
    ZEC_BTC,
    ZEC_ETH,
    ZEC_EUR,
    ZEC_USD,
    ZEC_USDT,
    ZEC_XMR,
}
