use bidir_map::BidirMap;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error::*;
use types::Currency;
use types::Pair;
use types::Pair::*;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(_1ST_BTC, "BTC-1ST");
        m.insert(_2GIVE_BTC, "BTC-2GIVE");
        m.insert(ABY_BTC, "BTC-ABY");
        m.insert(ADA_BTC, "BTC-ADA");
        m.insert(ADT_BTC, "BTC-ADT");
        m.insert(ADX_BTC, "BTC-ADX");
        m.insert(AEON_BTC, "BTC-AEON");
        m.insert(AGRS_BTC, "BTC-AGRS");
        m.insert(AMP_BTC, "BTC-AMP");
        m.insert(ANT_BTC, "BTC-ANT");
        m.insert(APX_BTC, "BTC-APX");
        m.insert(ARDR_BTC, "BTC-ARDR");
        m.insert(ARK_BTC, "BTC-ARK");
        m.insert(AUR_BTC, "BTC-AUR");
        m.insert(BAT_BTC, "BTC-BAT");
        m.insert(BAY_BTC, "BTC-BAY");
        m.insert(BCC_BTC, "BTC-BCC");
        m.insert(BCY_BTC, "BTC-BCY");
        m.insert(BITB_BTC, "BTC-BITB");
        m.insert(BLITZ_BTC, "BTC-BLITZ");
        m.insert(BLK_BTC, "BTC-BLK");
        m.insert(BLOCK_BTC, "BTC-BLOCK");
        m.insert(BNT_BTC, "BTC-BNT");
        m.insert(BRK_BTC, "BTC-BRK");
        m.insert(BRX_BTC, "BTC-BRX");
        m.insert(BSD_BTC, "BTC-BSD");
        m.insert(BTCD_BTC, "BTC-BTCD");
        m.insert(BTS_BTC, "BTC-BTS");
        m.insert(BURST_BTC, "BTC-BURST");
        m.insert(BYC_BTC, "BTC-BYC");
        m.insert(CANN_BTC, "BTC-CANN");
        m.insert(CFI_BTC, "BTC-CFI");
        m.insert(CLAM_BTC, "BTC-CLAM");
        m.insert(CLOAK_BTC, "BTC-CLOAK");
        m.insert(CLUB_BTC, "BTC-CLUB");
        m.insert(COVAL_BTC, "BTC-COVAL");
        m.insert(CPC_BTC, "BTC-CPC");
        m.insert(CRB_BTC, "BTC-CRB");
        m.insert(CRW_BTC, "BTC-CRW");
        m.insert(CURE_BTC, "BTC-CURE");
        m.insert(CVC_BTC, "BTC-CVC");
        m.insert(DASH_BTC, "BTC-DASH");
        m.insert(DCR_BTC, "BTC-DCR");
        m.insert(DCT_BTC, "BTC-DCT");
        m.insert(DGB_BTC, "BTC-DGB");
        m.insert(DGD_BTC, "BTC-DGD");
        m.insert(DMD_BTC, "BTC-DMD");
        m.insert(DNT_BTC, "BTC-DNT");
        m.insert(DOGE_BTC, "BTC-DOGE");
        m.insert(DOPE_BTC, "BTC-DOPE");
        m.insert(DTB_BTC, "BTC-DTB");
        m.insert(DYN_BTC, "BTC-DYN");
        m.insert(EBST_BTC, "BTC-EBST");
        m.insert(EDG_BTC, "BTC-EDG");
        m.insert(EFL_BTC, "BTC-EFL");
        m.insert(EGC_BTC, "BTC-EGC");
        m.insert(EMC_BTC, "BTC-EMC");
        m.insert(EMC2_BTC, "BTC-EMC2");
        m.insert(ENRG_BTC, "BTC-ENRG");
        m.insert(ERC_BTC, "BTC-ERC");
        m.insert(ETC_BTC, "BTC-ETC");
        m.insert(ETH_BTC, "BTC-ETH");
        m.insert(EXCL_BTC, "BTC-EXCL");
        m.insert(EXP_BTC, "BTC-EXP");
        m.insert(FAIR_BTC, "BTC-FAIR");
        m.insert(FCT_BTC, "BTC-FCT");
        m.insert(FLDC_BTC, "BTC-FLDC");
        m.insert(FLO_BTC, "BTC-FLO");
        m.insert(FTC_BTC, "BTC-FTC");
        m.insert(FUN_BTC, "BTC-FUN");
        m.insert(GAM_BTC, "BTC-GAM");
        m.insert(GAME_BTC, "BTC-GAME");
        m.insert(GBG_BTC, "BTC-GBG");
        m.insert(GBYTE_BTC, "BTC-GBYTE");
        m.insert(GCR_BTC, "BTC-GCR");
        m.insert(GEO_BTC, "BTC-GEO");
        m.insert(GLD_BTC, "BTC-GLD");
        m.insert(GNO_BTC, "BTC-GNO");
        m.insert(GNT_BTC, "BTC-GNT");
        m.insert(GOLOS_BTC, "BTC-GOLOS");
        m.insert(GRC_BTC, "BTC-GRC");
        m.insert(GRS_BTC, "BTC-GRS");
        m.insert(GUP_BTC, "BTC-GUP");
        m.insert(HMQ_BTC, "BTC-HMQ");
        m.insert(INCNT_BTC, "BTC-INCNT");
        m.insert(INFX_BTC, "BTC-INFX");
        m.insert(IOC_BTC, "BTC-IOC");
        m.insert(ION_BTC, "BTC-ION");
        m.insert(IOP_BTC, "BTC-IOP");
        m.insert(KMD_BTC, "BTC-KMD");
        m.insert(KORE_BTC, "BTC-KORE");
        m.insert(LBC_BTC, "BTC-LBC");
        m.insert(LGD_BTC, "BTC-LGD");
        m.insert(LMC_BTC, "BTC-LMC");
        m.insert(LSK_BTC, "BTC-LSK");
        m.insert(LTC_BTC, "BTC-LTC");
        m.insert(LUN_BTC, "BTC-LUN");
        m.insert(MAID_BTC, "BTC-MAID");
        m.insert(MANA_BTC, "BTC-MANA");
        m.insert(MCO_BTC, "BTC-MCO");
        m.insert(MEME_BTC, "BTC-MEME");
        m.insert(MLN_BTC, "BTC-MLN");
        m.insert(MONA_BTC, "BTC-MONA");
        m.insert(MTL_BTC, "BTC-MTL");
        m.insert(MUE_BTC, "BTC-MUE");
        m.insert(MUSIC_BTC, "BTC-MUSIC");
        m.insert(MYST_BTC, "BTC-MYST");
        m.insert(NAV_BTC, "BTC-NAV");
        m.insert(NBT_BTC, "BTC-NBT");
        m.insert(NEO_BTC, "BTC-NEO");
        m.insert(NEOS_BTC, "BTC-NEOS");
        m.insert(NLG_BTC, "BTC-NLG");
        m.insert(NMR_BTC, "BTC-NMR");
        m.insert(NXC_BTC, "BTC-NXC");
        m.insert(NXS_BTC, "BTC-NXS");
        m.insert(NXT_BTC, "BTC-NXT");
        m.insert(OK_BTC, "BTC-OK");
        m.insert(OMG_BTC, "BTC-OMG");
        m.insert(OMNI_BTC, "BTC-OMNI");
        m.insert(PART_BTC, "BTC-PART");
        m.insert(PAY_BTC, "BTC-PAY");
        m.insert(PDC_BTC, "BTC-PDC");
        m.insert(PINK_BTC, "BTC-PINK");
        m.insert(PIVX_BTC, "BTC-PIVX");
        m.insert(PKB_BTC, "BTC-PKB");
        m.insert(POT_BTC, "BTC-POT");
        m.insert(PPC_BTC, "BTC-PPC");
        m.insert(PTC_BTC, "BTC-PTC");
        m.insert(PTOY_BTC, "BTC-PTOY");
        m.insert(QRL_BTC, "BTC-QRL");
        m.insert(QTUM_BTC, "BTC-QTUM");
        m.insert(QWARK_BTC, "BTC-QWARK");
        m.insert(RADS_BTC, "BTC-RADS");
        m.insert(RBY_BTC, "BTC-RBY");
        m.insert(RDD_BTC, "BTC-RDD");
        m.insert(REP_BTC, "BTC-REP");
        m.insert(RISE_BTC, "BTC-RISE");
        m.insert(RLC_BTC, "BTC-RLC");
        m.insert(SAFEX_BTC, "BTC-SAFEX");
        m.insert(SALT_BTC, "BTC-SALT");
        m.insert(SBD_BTC, "BTC-SBD");
        m.insert(SC_BTC, "BTC-SC");
        m.insert(SEQ_BTC, "BTC-SEQ");
        m.insert(SHIFT_BTC, "BTC-SHIFT");
        m.insert(SIB_BTC, "BTC-SIB");
        m.insert(SLR_BTC, "BTC-SLR");
        m.insert(SLS_BTC, "BTC-SLS");
        m.insert(SNGLS_BTC, "BTC-SNGLS");
        m.insert(SNRG_BTC, "BTC-SNRG");
        m.insert(SNT_BTC, "BTC-SNT");
        m.insert(SPHR_BTC, "BTC-SPHR");
        m.insert(SPR_BTC, "BTC-SPR");
        m.insert(START_BTC, "BTC-START");
        m.insert(STEEM_BTC, "BTC-STEEM");
        m.insert(STORJ_BTC, "BTC-STORJ");
        m.insert(STRAT_BTC, "BTC-STRAT");
        m.insert(SWIFT_BTC, "BTC-SWIFT");
        m.insert(SWT_BTC, "BTC-SWT");
        m.insert(SYNX_BTC, "BTC-SYNX");
        m.insert(SYS_BTC, "BTC-SYS");
        m.insert(THC_BTC, "BTC-THC");
        m.insert(TIME_BTC, "BTC-TIME");
        m.insert(TIX_BTC, "BTC-TIX");
        m.insert(TKN_BTC, "BTC-TKN");
        m.insert(TKS_BTC, "BTC-TKS");
        m.insert(TRIG_BTC, "BTC-TRIG");
        m.insert(TRST_BTC, "BTC-TRST");
        m.insert(TRUST_BTC, "BTC-TRUST");
        m.insert(TX_BTC, "BTC-TX");
        m.insert(UBQ_BTC, "BTC-UBQ");
        m.insert(UNB_BTC, "BTC-UNB");
        m.insert(VIA_BTC, "BTC-VIA");
        m.insert(VOX_BTC, "BTC-VOX");
        m.insert(VRC_BTC, "BTC-VRC");
        m.insert(VRM_BTC, "BTC-VRM");
        m.insert(VTC_BTC, "BTC-VTC");
        m.insert(VTR_BTC, "BTC-VTR");
        m.insert(WAVES_BTC, "BTC-WAVES");
        m.insert(WINGS_BTC, "BTC-WINGS");
        m.insert(XAUR_BTC, "BTC-XAUR");
        m.insert(XCP_BTC, "BTC-XCP");
        m.insert(XDN_BTC, "BTC-XDN");
        m.insert(XEL_BTC, "BTC-XEL");
        m.insert(XEM_BTC, "BTC-XEM");
        m.insert(XLM_BTC, "BTC-XLM");
        m.insert(XMG_BTC, "BTC-XMG");
        m.insert(XMR_BTC, "BTC-XMR");
        m.insert(XMY_BTC, "BTC-XMY");
        m.insert(XRP_BTC, "BTC-XRP");
        m.insert(XST_BTC, "BTC-XST");
        m.insert(XVC_BTC, "BTC-XVC");
        m.insert(XVG_BTC, "BTC-XVG");
        m.insert(XWC_BTC, "BTC-XWC");
        m.insert(XZC_BTC, "BTC-XZC");
        m.insert(ZCL_BTC, "BTC-ZCL");
        m.insert(ZEC_BTC, "BTC-ZEC");
        m.insert(ZEN_BTC, "BTC-ZEN");
        m.insert(1ST_ETH, "ETH-1ST");
        m.insert(ADT_ETH, "ETH-ADT");
        m.insert(ADX_ETH, "ETH-ADX");
        m.insert(ANT_ETH, "ETH-ANT");
        m.insert(BAT_ETH, "ETH-BAT");
        m.insert(BCC_ETH, "ETH-BCC");
        m.insert(BNT_ETH, "ETH-BNT");
        m.insert(BTS_ETH, "ETH-BTS");
        m.insert(CFI_ETH, "ETH-CFI");
        m.insert(CRB_ETH, "ETH-CRB");
        m.insert(CVC_ETH, "ETH-CVC");
        m.insert(DASH_ETH, "ETH-DASH");
        m.insert(DGB_ETH, "ETH-DGB");
        m.insert(DGD_ETH, "ETH-DGD");
        m.insert(DNT_ETH, "ETH-DNT");
        m.insert(ETC_ETH, "ETH-ETC");
        m.insert(FCT_ETH, "ETH-FCT");
        m.insert(FUN_ETH, "ETH-FUN");
        m.insert(GNO_ETH, "ETH-GNO");
        m.insert(GNT_ETH, "ETH-GNT");
        m.insert(GUP_ETH, "ETH-GUP");
        m.insert(HMQ_ETH, "ETH-HMQ");
        m.insert(LGD_ETH, "ETH-LGD");
        m.insert(LTC_ETH, "ETH-LTC");
        m.insert(LUN_ETH, "ETH-LUN");
        m.insert(MANA_ETH, "ETH-MANA");
        m.insert(MCO_ETH, "ETH-MCO");
        m.insert(MTL_ETH, "ETH-MTL");
        m.insert(MYST_ETH, "ETH-MYST");
        m.insert(NEO_ETH, "ETH-NEO");
        m.insert(NMR_ETH, "ETH-NMR");
        m.insert(OMG_ETH, "ETH-OMG");
        m.insert(PAY_ETH, "ETH-PAY");
        m.insert(PTOY_ETH, "ETH-PTOY");
        m.insert(QRL_ETH, "ETH-QRL");
        m.insert(QTUM_ETH, "ETH-QTUM");
        m.insert(REP_ETH, "ETH-REP");
        m.insert(RLC_ETH, "ETH-RLC");
        m.insert(SALT_ETH, "ETH-SALT");
        m.insert(SC_ETH, "ETH-SC");
        m.insert(SNGLS_ETH, "ETH-SNGLS");
        m.insert(SNT_ETH, "ETH-SNT");
        m.insert(STORJ_ETH, "ETH-STORJ");
        m.insert(STRAT_ETH, "ETH-STRAT");
        m.insert(TIME_ETH, "ETH-TIME");
        m.insert(TIX_ETH, "ETH-TIX");
        m.insert(TKN_ETH, "ETH-TKN");
        m.insert(TRST_ETH, "ETH-TRST");
        m.insert(WAVES_ETH, "ETH-WAVES");
        m.insert(WINGS_ETH, "ETH-WINGS");
        m.insert(XEM_ETH, "ETH-XEM");
        m.insert(XLM_ETH, "ETH-XLM");
        m.insert(XMR_ETH, "ETH-XMR");
        m.insert(XRP_ETH, "ETH-XRP");
        m.insert(ZEC_ETH, "ETH-ZEC");
        m.insert(BCC_USDT, "USDT-BCC");
        m.insert(BTC_USDT, "USDT-BTC");
        m.insert(DASH_USDT, "USDT-DASH");
        m.insert(ETC_USDT, "USDT-ETC");
        m.insert(ETH_USDT, "USDT-ETH");
        m.insert(LTC_USDT, "USDT-LTC");
        m.insert(NEO_USDT, "USDT-NEO");
        m.insert(OMG_USDT, "USDT-OMG");
        m.insert(XMR_USDT, "USDT-XMR");
        m.insert(XRP_USDT, "USDT-XRP");
        m.insert(ZEC_USDT, "USDT-ZEC");
        m
    };
}

/// Return the name associated to pair used by Bittrex
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: &Pair) -> Option<&&str> {
    PAIRS_STRING.get_by_first(pair)
}

/// Return the Pair enum associated to the string used by Bittrex
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

/// If error array is null, return the result (encoded in a json object)
/// else return the error string found in array
pub fn parse_result(response: &Map<String, Value>) -> Result<Map<String, Value>> {
    let error_array = match response.get("error") {
        Some(array) => {
            array
                .as_array()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat("error".to_string()))?
        }
        None => return Err(ErrorKind::BadParse.into()),
    };
    if error_array.is_empty() {
        return Ok(response
                      .get("result")
                      .ok_or_else(|| ErrorKind::MissingField("result".to_string()))?
                      .as_object()
                      .ok_or_else(|| ErrorKind::InvalidFieldFormat("result".to_string()))?
                      .clone());
    }
    let error_msg = error_array[0]
        .as_str()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat(error_array[0].to_string()))?
        .to_string();

    //TODO: Parse correctly the reason for "EService:Unavailable".
    match error_msg.as_ref() {
        "EService:Unavailable" => {
            Err(ErrorKind::ServiceUnavailable("Unknown...".to_string()).into())
        }
        "EAPI:Invalid key" => Err(ErrorKind::BadCredentials.into()),
        "EAPI:Invalid nonce" => Err(ErrorKind::InvalidNonce.into()),
        "EOrder:Rate limit exceeded" => Err(ErrorKind::RateLimitExceeded.into()),
        "EQuery:Unknown asset pair" => Err(ErrorKind::PairUnsupported.into()),
        "EGeneral:Invalid arguments" => Err(ErrorKind::InvalidArguments.into()),
        "EGeneral:Permission denied" => Err(ErrorKind::PermissionDenied.into()),
        "EOrder:Insufficient funds" => Err(ErrorKind::InsufficientFunds.into()),
        "EOrder:Order minimum not met" => Err(ErrorKind::InsufficientOrderSize.into()),
        other => Err(ErrorKind::ExchangeSpecificError(other.to_string()).into()),
    }
}

/// Return the currency enum associated with the
/// string used by Bittrex. If no currency is found,
/// return None
/// # Examples
///
/// ```
/// use coinnect::bittrex::utils::get_currency_enum;
/// use coinnect::types::Currency;
///
/// let currency = get_currency_enum("ZUSD");
/// assert_eq!(Some(Currency::USD), currency);
/// ```
pub fn get_currency_enum(currency: &str) -> Option<Currency> {
    match currency {
        "ZEUR" => Some(Currency::EUR),
        "ZCAD" => Some(Currency::CAD),
        "ZGBP" => Some(Currency::GBP),
        "ZJPY" => Some(Currency::JPY),
        "ZUSD" => Some(Currency::USD),
        "XDASH" => Some(Currency::DASH),
        "XETC" => Some(Currency::ETC),
        "XETH" => Some(Currency::ETH),
        "XGNO" => Some(Currency::GNO),
        "XICN" => Some(Currency::ICN),
        "XLTC" => Some(Currency::LTC),
        "XMLN" => Some(Currency::MLN),
        "XREP" => Some(Currency::REP),
        "XUSDT" => Some(Currency::USDT),
        "XXBT" => Some(Currency::BTC),
        "XXDG" => Some(Currency::XDG),
        "XXLM" => Some(Currency::XLM),
        "XXMR" => Some(Currency::XMR),
        "XXRP" => Some(Currency::XRP),
        "XZEC" => Some(Currency::ZEC),
        _ => None,
    }
}

/// Return the currency String associated with the
/// string used by Bittrex. If no currency is found,
/// return None
/// # Examples
///
/// ```
/// use coinnect::bittrex::utils::get_currency_string;
/// use coinnect::types::Currency;
///
/// let currency = get_currency_string(Currency::BTC);
/// assert_eq!(currency, Some("XXBT".to_string()));
/// ```
pub fn get_currency_string(currency: Currency) -> Option<String> {
    match currency {
        Currency::EUR => Some("ZEUR".to_string()),
        Currency::CAD => Some("ZCAD".to_string()),
        Currency::GBP => Some("ZGBP".to_string()),
        Currency::JPY => Some("ZJPY".to_string()),
        Currency::USD => Some("ZUSD".to_string()),
        Currency::DASH => Some("XDASH".to_string()),
        Currency::ETC => Some("XETC".to_string()),
        Currency::ETH => Some("XETH".to_string()),
        Currency::GNO => Some("XGNO".to_string()),
        Currency::ICN => Some("XICN".to_string()),
        Currency::LTC => Some("XLTC".to_string()),
        Currency::MLN => Some("XMLN".to_string()),
        Currency::REP => Some("XREP".to_string()),
        Currency::USDT => Some("XUSDT".to_string()),
        Currency::BTC => Some("XXBT".to_string()),
        Currency::XDG => Some("XXDG".to_string()),
        Currency::XLM => Some("XXLM".to_string()),
        Currency::XMR => Some("XXMR".to_string()),
        Currency::XRP => Some("XXRP".to_string()),
        Currency::ZEC => Some("XZEC".to_string()),
        _ => None,
    }
}
