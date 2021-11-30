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
        m.insert(_1ST_ETH, "ETH-1ST");
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

/// If error array is null, return the result (which can be an array, object or null)
/// else return the error string found in array
pub fn parse_result(response: &Map<String, Value>) -> Result<Value> {
    let is_success = match response["success"].as_bool() {
        Some(is_success) => {
            is_success
        }
        None => return Err(ErrorKind::BadParse.into()),
    };

    if is_success {
        Ok(response.get("result").unwrap().clone())
    }
    else {
        let error_message = response.get("message")
        .ok_or_else(|| ErrorKind::MissingField("message".to_string()))?
        .as_str()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat("message".to_string()))?;

        match error_message.as_ref() {
            "MIN_TRADE_REQUIREMENT_NOT_MET" => Err(ErrorKind::InsufficientOrderSize.into()),
            "INVALID_PERMISSION" => Err(ErrorKind::PermissionDenied.into()),
            _ => Err(ErrorKind::ExchangeSpecificError(error_message.to_string()).into()),
        }
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
/// let currency = get_currency_enum("1ST");
/// assert_eq!(Some(Currency::_1ST), currency);
/// ```
pub fn get_currency_enum(currency: &str) -> Option<Currency> {
    match currency {
        "1ST" => Some(Currency::_1ST),
        "2GIVE" => Some(Currency::_2GIVE),
        "8BIT" => Some(Currency::_8BIT),
        "ABY" => Some(Currency::ABY),
        "ADA" => Some(Currency::ADA),
        "ADC" => Some(Currency::ADC),
        "ADT" => Some(Currency::ADT),
        "ADX" => Some(Currency::ADX),
        "AEON" => Some(Currency::AEON),
        "AGRS" => Some(Currency::AGRS),
        "AM" => Some(Currency::AM),
        "AMP" => Some(Currency::AMP),
        "AMS" => Some(Currency::AMS),
        "ANT" => Some(Currency::ANT),
        "APEX" => Some(Currency::APEX),
        "APX" => Some(Currency::APX),
        "ARB" => Some(Currency::ARB),
        "ARDR" => Some(Currency::ARDR),
        "ARK" => Some(Currency::ARK),
        "AUR" => Some(Currency::AUR),
        "BAT" => Some(Currency::BAT),
        "BAY" => Some(Currency::BAY),
        "BCC" => Some(Currency::BCC),
        "BCY" => Some(Currency::BCY),
        "BITB" => Some(Currency::BITB),
        "BITCNY" => Some(Currency::BITCNY),
        "BITS" => Some(Currency::BITS),
        "BITZ" => Some(Currency::BITZ),
        "BLC" => Some(Currency::BLC),
        "BLITZ" => Some(Currency::BLITZ),
        "BLK" => Some(Currency::BLK),
        "BLOCK" => Some(Currency::BLOCK),
        "BNT" => Some(Currency::BNT),
        "BOB" => Some(Currency::BOB),
        "BRK" => Some(Currency::BRK),
        "BRX" => Some(Currency::BRX),
        "BSD" => Some(Currency::BSD),
        "BSTY" => Some(Currency::BSTY),
        "BTA" => Some(Currency::BTA),
        "BTC" => Some(Currency::BTC),
        "BTCD" => Some(Currency::BTCD),
        "BTS" => Some(Currency::BTS),
        "BURST" => Some(Currency::BURST),
        "BYC" => Some(Currency::BYC),
        "CANN" => Some(Currency::CANN),
        "CCN" => Some(Currency::CCN),
        "CFI" => Some(Currency::CFI),
        "CLAM" => Some(Currency::CLAM),
        "CLOAK" => Some(Currency::CLOAK),
        "CLUB" => Some(Currency::CLUB),
        "COVAL" => Some(Currency::COVAL),
        "CPC" => Some(Currency::CPC),
        "CRB" => Some(Currency::CRB),
        "CRBIT" => Some(Currency::CRBIT),
        "CRW" => Some(Currency::CRW),
        "CRYPT" => Some(Currency::CRYPT),
        "CURE" => Some(Currency::CURE),
        "CVC" => Some(Currency::CVC),
        "DAR" => Some(Currency::DAR),
        "DASH" => Some(Currency::DASH),
        "DCR" => Some(Currency::DCR),
        "DCT" => Some(Currency::DCT),
        "DGB" => Some(Currency::DGB),
        "DGC" => Some(Currency::DGC),
        "DGD" => Some(Currency::DGD),
        "DMD" => Some(Currency::DMD),
        "DNT" => Some(Currency::DNT),
        "DOGE" => Some(Currency::DOGE),
        "DOPE" => Some(Currency::DOPE),
        "DRACO" => Some(Currency::DRACO),
        "DTB" => Some(Currency::DTB),
        "DTC" => Some(Currency::DTC),
        "DYN" => Some(Currency::DYN),
        "EBST" => Some(Currency::EBST),
        "EDG" => Some(Currency::EDG),
        "EFL" => Some(Currency::EFL),
        "EGC" => Some(Currency::EGC),
        "EMC" => Some(Currency::EMC),
        "EMC2" => Some(Currency::EMC2),
        "ENRG" => Some(Currency::ENRG),
        "ERC" => Some(Currency::ERC),
        "ETC" => Some(Currency::ETC),
        "ETH" => Some(Currency::ETH),
        "EXCL" => Some(Currency::EXCL),
        "EXP" => Some(Currency::EXP),
        "FAIR" => Some(Currency::FAIR),
        "FC2" => Some(Currency::FC2),
        "FCT" => Some(Currency::FCT),
        "FLDC" => Some(Currency::FLDC),
        "FLO" => Some(Currency::FLO),
        "FRK" => Some(Currency::FRK),
        "FSC2" => Some(Currency::FSC2),
        "FTC" => Some(Currency::FTC),
        "FUN" => Some(Currency::FUN),
        "GAM" => Some(Currency::GAM),
        "GAME" => Some(Currency::GAME),
        "GBG" => Some(Currency::GBG),
        "GBYTE" => Some(Currency::GBYTE),
        "GCR" => Some(Currency::GCR),
        "GEMZ" => Some(Currency::GEMZ),
        "GEO" => Some(Currency::GEO),
        "GHC" => Some(Currency::GHC),
        "GLD" => Some(Currency::GLD),
        "GNO" => Some(Currency::GNO),
        "GNT" => Some(Currency::GNT),
        "GOLOS" => Some(Currency::GOLOS),
        "GP" => Some(Currency::GP),
        "GRC" => Some(Currency::GRC),
        "GRS" => Some(Currency::GRS),
        "GRT" => Some(Currency::GRT),
        "GUP" => Some(Currency::GUP),
        "HKG" => Some(Currency::HKG),
        "HMQ" => Some(Currency::HMQ),
        "HYPER" => Some(Currency::HYPER),
        "HZ" => Some(Currency::HZ),
        "INCNT" => Some(Currency::INCNT),
        "INFX" => Some(Currency::INFX),
        "IOC" => Some(Currency::IOC),
        "ION" => Some(Currency::ION),
        "IOP" => Some(Currency::IOP),
        "J" => Some(Currency::J),
        "KMD" => Some(Currency::KMD),
        "KORE" => Some(Currency::KORE),
        "KR" => Some(Currency::KR),
        "LBC" => Some(Currency::LBC),
        "LGD" => Some(Currency::LGD),
        "LMC" => Some(Currency::LMC),
        "LSK" => Some(Currency::LSK),
        "LTC" => Some(Currency::LTC),
        "LUN" => Some(Currency::LUN),
        "LXC" => Some(Currency::LXC),
        "MAID" => Some(Currency::MAID),
        "MANA" => Some(Currency::MANA),
        "MAX" => Some(Currency::MAX),
        "MCO" => Some(Currency::MCO),
        "MEC" => Some(Currency::MEC),
        "MEME" => Some(Currency::MEME),
        "METAL" => Some(Currency::METAL),
        "MLN" => Some(Currency::MLN),
        "MND" => Some(Currency::MND),
        "MONA" => Some(Currency::MONA),
        "MTL" => Some(Currency::MTL),
        "MTR" => Some(Currency::MTR),
        "MUE" => Some(Currency::MUE),
        "MUSIC" => Some(Currency::MUSIC),
        "MYST" => Some(Currency::MYST),
        "MZC" => Some(Currency::MZC),
        "NAUT" => Some(Currency::NAUT),
        "NAV" => Some(Currency::NAV),
        "NBT" => Some(Currency::NBT),
        "NEO" => Some(Currency::NEO),
        "NEOS" => Some(Currency::NEOS),
        "NET" => Some(Currency::NET),
        "NEU" => Some(Currency::NEU),
        "NLG" => Some(Currency::NLG),
        "NMR" => Some(Currency::NMR),
        "NTRN" => Some(Currency::NTRN),
        "NXC" => Some(Currency::NXC),
        "NXS" => Some(Currency::NXS),
        "NXT" => Some(Currency::NXT),
        "OC" => Some(Currency::OC),
        "OK" => Some(Currency::OK),
        "OMG" => Some(Currency::OMG),
        "OMNI" => Some(Currency::OMNI),
        "ORB" => Some(Currency::ORB),
        "PART" => Some(Currency::PART),
        "PAY" => Some(Currency::PAY),
        "PDC" => Some(Currency::PDC),
        "PINK" => Some(Currency::PINK),
        "PIVX" => Some(Currency::PIVX),
        "PKB" => Some(Currency::PKB),
        "POT" => Some(Currency::POT),
        "PPC" => Some(Currency::PPC),
        "PRIME" => Some(Currency::PRIME),
        "PTC" => Some(Currency::PTC),
        "PTOY" => Some(Currency::PTOY),
        "PXI" => Some(Currency::PXI),
        "QRL" => Some(Currency::QRL),
        "QTUM" => Some(Currency::QTUM),
        "QWARK" => Some(Currency::QWARK),
        "RADS" => Some(Currency::RADS),
        "RBY" => Some(Currency::RBY),
        "RDD" => Some(Currency::RDD),
        "REP" => Some(Currency::REP),
        "RISE" => Some(Currency::RISE),
        "RLC" => Some(Currency::RLC),
        "ROOT" => Some(Currency::ROOT),
        "SAFEX" => Some(Currency::SAFEX),
        "SALT" => Some(Currency::SALT),
        "SBD" => Some(Currency::SBD),
        "SC" => Some(Currency::SC),
        "SCOT" => Some(Currency::SCOT),
        "SCRT" => Some(Currency::SCRT),
        "SEQ" => Some(Currency::SEQ),
        "SFR" => Some(Currency::SFR),
        "SHIFT" => Some(Currency::SHIFT),
        "SIB" => Some(Currency::SIB),
        "SLG" => Some(Currency::SLG),
        "SLING" => Some(Currency::SLING),
        "SLR" => Some(Currency::SLR),
        "SLS" => Some(Currency::SLS),
        "SNGLS" => Some(Currency::SNGLS),
        "SNRG" => Some(Currency::SNRG),
        "SNT" => Some(Currency::SNT),
        "SOON" => Some(Currency::SOON),
        "SPHR" => Some(Currency::SPHR),
        "SPR" => Some(Currency::SPR),
        "SPRTS" => Some(Currency::SPRTS),
        "SSD" => Some(Currency::SSD),
        "START" => Some(Currency::START),
        "STEEM" => Some(Currency::STEEM),
        "STEPS" => Some(Currency::STEPS),
        "STORJ" => Some(Currency::STORJ),
        "STRAT" => Some(Currency::STRAT),
        "STV" => Some(Currency::STV),
        "SWIFT" => Some(Currency::SWIFT),
        "SWING" => Some(Currency::SWING),
        "SWT" => Some(Currency::SWT),
        "SYNX" => Some(Currency::SYNX),
        "SYS" => Some(Currency::SYS),
        "TES" => Some(Currency::TES),
        "THC" => Some(Currency::THC),
        "TIME" => Some(Currency::TIME),
        "TIT" => Some(Currency::TIT),
        "TIX" => Some(Currency::TIX),
        "TKN" => Some(Currency::TKN),
        "TKS" => Some(Currency::TKS),
        "TRI" => Some(Currency::TRI),
        "TRIG" => Some(Currency::TRIG),
        "TRK" => Some(Currency::TRK),
        "TROLL" => Some(Currency::TROLL),
        "TRST" => Some(Currency::TRST),
        "TRUST" => Some(Currency::TRUST),
        "TX" => Some(Currency::TX),
        "U" => Some(Currency::U),
        "UBQ" => Some(Currency::UBQ),
        "UFO" => Some(Currency::UFO),
        "UNB" => Some(Currency::UNB),
        "UNIQ" => Some(Currency::UNIQ),
        "UNIT" => Some(Currency::UNIT),
        "UNO" => Some(Currency::UNO),
        "USDT" => Some(Currency::USDT),
        "UTC" => Some(Currency::UTC),
        "VIA" => Some(Currency::VIA),
        "VIOR" => Some(Currency::VIOR),
        "VIRAL" => Some(Currency::VIRAL),
        "VOX" => Some(Currency::VOX),
        "VPN" => Some(Currency::VPN),
        "VRC" => Some(Currency::VRC),
        "VRM" => Some(Currency::VRM),
        "VTC" => Some(Currency::VTC),
        "VTR" => Some(Currency::VTR),
        "WARP" => Some(Currency::WARP),
        "WAVES" => Some(Currency::WAVES),
        "WINGS" => Some(Currency::WINGS),
        "XAUR" => Some(Currency::XAUR),
        "XBB" => Some(Currency::XBB),
        "XC" => Some(Currency::XC),
        "XCO" => Some(Currency::XCO),
        "XCP" => Some(Currency::XCP),
        "XDN" => Some(Currency::XDN),
        "XDQ" => Some(Currency::XDQ),
        "XEL" => Some(Currency::XEL),
        "XEM" => Some(Currency::XEM),
        "XLM" => Some(Currency::XLM),
        "XMG" => Some(Currency::XMG),
        "XMR" => Some(Currency::XMR),
        "XMY" => Some(Currency::XMY),
        "XPY" => Some(Currency::XPY),
        "XQN" => Some(Currency::XQN),
        "XRP" => Some(Currency::XRP),
        "XSEED" => Some(Currency::XSEED),
        "XST" => Some(Currency::XST),
        "XTC" => Some(Currency::XTC),
        "XVC" => Some(Currency::XVC),
        "XVG" => Some(Currency::XVG),
        "XWC" => Some(Currency::XWC),
        "XZC" => Some(Currency::XZC),
        "YBC" => Some(Currency::YBC),
        "ZCL" => Some(Currency::ZCL),
        "ZEC" => Some(Currency::ZEC),
        "ZEN" => Some(Currency::ZEN),
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
/// let currency = get_currency_string(Currency::_1ST);
/// assert_eq!(currency, Some("1ST".to_string()));
/// ```
pub fn get_currency_string(currency: Currency) -> Option<String> {
    match currency {
    Currency::_1ST => Some("1ST".to_string()),
    Currency::_2GIVE => Some("2GIVE".to_string()),
    Currency::_8BIT => Some("8BIT".to_string()),
    Currency::ABY => Some("ABY".to_string()),
    Currency::ADA => Some("ADA".to_string()),
    Currency::ADC => Some("ADC".to_string()),
    Currency::ADT => Some("ADT".to_string()),
    Currency::ADX => Some("ADX".to_string()),
    Currency::AEON => Some("AEON".to_string()),
    Currency::AGRS => Some("AGRS".to_string()),
    Currency::AM => Some("AM".to_string()),
    Currency::AMP => Some("AMP".to_string()),
    Currency::AMS => Some("AMS".to_string()),
    Currency::ANT => Some("ANT".to_string()),
    Currency::APEX => Some("APEX".to_string()),
    Currency::APX => Some("APX".to_string()),
    Currency::ARB => Some("ARB".to_string()),
    Currency::ARDR => Some("ARDR".to_string()),
    Currency::ARK => Some("ARK".to_string()),
    Currency::AUR => Some("AUR".to_string()),
    Currency::BAT => Some("BAT".to_string()),
    Currency::BAY => Some("BAY".to_string()),
    Currency::BCC => Some("BCC".to_string()),
    Currency::BCY => Some("BCY".to_string()),
    Currency::BITB => Some("BITB".to_string()),
    Currency::BITCNY => Some("BITCNY".to_string()),
    Currency::BITS => Some("BITS".to_string()),
    Currency::BITZ => Some("BITZ".to_string()),
    Currency::BLC => Some("BLC".to_string()),
    Currency::BLITZ => Some("BLITZ".to_string()),
    Currency::BLK => Some("BLK".to_string()),
    Currency::BLOCK => Some("BLOCK".to_string()),
    Currency::BNT => Some("BNT".to_string()),
    Currency::BOB => Some("BOB".to_string()),
    Currency::BRK => Some("BRK".to_string()),
    Currency::BRX => Some("BRX".to_string()),
    Currency::BSD => Some("BSD".to_string()),
    Currency::BSTY => Some("BSTY".to_string()),
    Currency::BTA => Some("BTA".to_string()),
    Currency::BTC => Some("BTC".to_string()),
    Currency::BTCD => Some("BTCD".to_string()),
    Currency::BTS => Some("BTS".to_string()),
    Currency::BURST => Some("BURST".to_string()),
    Currency::BYC => Some("BYC".to_string()),
    Currency::CANN => Some("CANN".to_string()),
    Currency::CCN => Some("CCN".to_string()),
    Currency::CFI => Some("CFI".to_string()),
    Currency::CLAM => Some("CLAM".to_string()),
    Currency::CLOAK => Some("CLOAK".to_string()),
    Currency::CLUB => Some("CLUB".to_string()),
    Currency::COVAL => Some("COVAL".to_string()),
    Currency::CPC => Some("CPC".to_string()),
    Currency::CRB => Some("CRB".to_string()),
    Currency::CRBIT => Some("CRBIT".to_string()),
    Currency::CRW => Some("CRW".to_string()),
    Currency::CRYPT => Some("CRYPT".to_string()),
    Currency::CURE => Some("CURE".to_string()),
    Currency::CVC => Some("CVC".to_string()),
    Currency::DAR => Some("DAR".to_string()),
    Currency::DASH => Some("DASH".to_string()),
    Currency::DCR => Some("DCR".to_string()),
    Currency::DCT => Some("DCT".to_string()),
    Currency::DGB => Some("DGB".to_string()),
    Currency::DGC => Some("DGC".to_string()),
    Currency::DGD => Some("DGD".to_string()),
    Currency::DMD => Some("DMD".to_string()),
    Currency::DNT => Some("DNT".to_string()),
    Currency::DOGE => Some("DOGE".to_string()),
    Currency::DOPE => Some("DOPE".to_string()),
    Currency::DRACO => Some("DRACO".to_string()),
    Currency::DTB => Some("DTB".to_string()),
    Currency::DTC => Some("DTC".to_string()),
    Currency::DYN => Some("DYN".to_string()),
    Currency::EBST => Some("EBST".to_string()),
    Currency::EDG => Some("EDG".to_string()),
    Currency::EFL => Some("EFL".to_string()),
    Currency::EGC => Some("EGC".to_string()),
    Currency::EMC => Some("EMC".to_string()),
    Currency::EMC2 => Some("EMC2".to_string()),
    Currency::ENRG => Some("ENRG".to_string()),
    Currency::ERC => Some("ERC".to_string()),
    Currency::ETC => Some("ETC".to_string()),
    Currency::ETH => Some("ETH".to_string()),
    Currency::EXCL => Some("EXCL".to_string()),
    Currency::EXP => Some("EXP".to_string()),
    Currency::FAIR => Some("FAIR".to_string()),
    Currency::FC2 => Some("FC2".to_string()),
    Currency::FCT => Some("FCT".to_string()),
    Currency::FLDC => Some("FLDC".to_string()),
    Currency::FLO => Some("FLO".to_string()),
    Currency::FRK => Some("FRK".to_string()),
    Currency::FSC2 => Some("FSC2".to_string()),
    Currency::FTC => Some("FTC".to_string()),
    Currency::FUN => Some("FUN".to_string()),
    Currency::GAM => Some("GAM".to_string()),
    Currency::GAME => Some("GAME".to_string()),
    Currency::GBG => Some("GBG".to_string()),
    Currency::GBYTE => Some("GBYTE".to_string()),
    Currency::GCR => Some("GCR".to_string()),
    Currency::GEMZ => Some("GEMZ".to_string()),
    Currency::GEO => Some("GEO".to_string()),
    Currency::GHC => Some("GHC".to_string()),
    Currency::GLD => Some("GLD".to_string()),
    Currency::GNO => Some("GNO".to_string()),
    Currency::GNT => Some("GNT".to_string()),
    Currency::GOLOS => Some("GOLOS".to_string()),
    Currency::GP => Some("GP".to_string()),
    Currency::GRC => Some("GRC".to_string()),
    Currency::GRS => Some("GRS".to_string()),
    Currency::GRT => Some("GRT".to_string()),
    Currency::GUP => Some("GUP".to_string()),
    Currency::HKG => Some("HKG".to_string()),
    Currency::HMQ => Some("HMQ".to_string()),
    Currency::HYPER => Some("HYPER".to_string()),
    Currency::HZ => Some("HZ".to_string()),
    Currency::INCNT => Some("INCNT".to_string()),
    Currency::INFX => Some("INFX".to_string()),
    Currency::IOC => Some("IOC".to_string()),
    Currency::ION => Some("ION".to_string()),
    Currency::IOP => Some("IOP".to_string()),
    Currency::J => Some("J".to_string()),
    Currency::KMD => Some("KMD".to_string()),
    Currency::KORE => Some("KORE".to_string()),
    Currency::KR => Some("KR".to_string()),
    Currency::LBC => Some("LBC".to_string()),
    Currency::LGD => Some("LGD".to_string()),
    Currency::LMC => Some("LMC".to_string()),
    Currency::LSK => Some("LSK".to_string()),
    Currency::LTC => Some("LTC".to_string()),
    Currency::LUN => Some("LUN".to_string()),
    Currency::LXC => Some("LXC".to_string()),
    Currency::MAID => Some("MAID".to_string()),
    Currency::MANA => Some("MANA".to_string()),
    Currency::MAX => Some("MAX".to_string()),
    Currency::MCO => Some("MCO".to_string()),
    Currency::MEC => Some("MEC".to_string()),
    Currency::MEME => Some("MEME".to_string()),
    Currency::METAL => Some("METAL".to_string()),
    Currency::MLN => Some("MLN".to_string()),
    Currency::MND => Some("MND".to_string()),
    Currency::MONA => Some("MONA".to_string()),
    Currency::MTL => Some("MTL".to_string()),
    Currency::MTR => Some("MTR".to_string()),
    Currency::MUE => Some("MUE".to_string()),
    Currency::MUSIC => Some("MUSIC".to_string()),
    Currency::MYST => Some("MYST".to_string()),
    Currency::MZC => Some("MZC".to_string()),
    Currency::NAUT => Some("NAUT".to_string()),
    Currency::NAV => Some("NAV".to_string()),
    Currency::NBT => Some("NBT".to_string()),
    Currency::NEO => Some("NEO".to_string()),
    Currency::NEOS => Some("NEOS".to_string()),
    Currency::NET => Some("NET".to_string()),
    Currency::NEU => Some("NEU".to_string()),
    Currency::NLG => Some("NLG".to_string()),
    Currency::NMR => Some("NMR".to_string()),
    Currency::NTRN => Some("NTRN".to_string()),
    Currency::NXC => Some("NXC".to_string()),
    Currency::NXS => Some("NXS".to_string()),
    Currency::NXT => Some("NXT".to_string()),
    Currency::OC => Some("OC".to_string()),
    Currency::OK => Some("OK".to_string()),
    Currency::OMG => Some("OMG".to_string()),
    Currency::OMNI => Some("OMNI".to_string()),
    Currency::ORB => Some("ORB".to_string()),
    Currency::PART => Some("PART".to_string()),
    Currency::PAY => Some("PAY".to_string()),
    Currency::PDC => Some("PDC".to_string()),
    Currency::PINK => Some("PINK".to_string()),
    Currency::PIVX => Some("PIVX".to_string()),
    Currency::PKB => Some("PKB".to_string()),
    Currency::POT => Some("POT".to_string()),
    Currency::PPC => Some("PPC".to_string()),
    Currency::PRIME => Some("PRIME".to_string()),
    Currency::PTC => Some("PTC".to_string()),
    Currency::PTOY => Some("PTOY".to_string()),
    Currency::PXI => Some("PXI".to_string()),
    Currency::QRL => Some("QRL".to_string()),
    Currency::QTUM => Some("QTUM".to_string()),
    Currency::QWARK => Some("QWARK".to_string()),
    Currency::RADS => Some("RADS".to_string()),
    Currency::RBY => Some("RBY".to_string()),
    Currency::RDD => Some("RDD".to_string()),
    Currency::REP => Some("REP".to_string()),
    Currency::RISE => Some("RISE".to_string()),
    Currency::RLC => Some("RLC".to_string()),
    Currency::ROOT => Some("ROOT".to_string()),
    Currency::SAFEX => Some("SAFEX".to_string()),
    Currency::SALT => Some("SALT".to_string()),
    Currency::SBD => Some("SBD".to_string()),
    Currency::SC => Some("SC".to_string()),
    Currency::SCOT => Some("SCOT".to_string()),
    Currency::SCRT => Some("SCRT".to_string()),
    Currency::SEQ => Some("SEQ".to_string()),
    Currency::SFR => Some("SFR".to_string()),
    Currency::SHIFT => Some("SHIFT".to_string()),
    Currency::SIB => Some("SIB".to_string()),
    Currency::SLG => Some("SLG".to_string()),
    Currency::SLING => Some("SLING".to_string()),
    Currency::SLR => Some("SLR".to_string()),
    Currency::SLS => Some("SLS".to_string()),
    Currency::SNGLS => Some("SNGLS".to_string()),
    Currency::SNRG => Some("SNRG".to_string()),
    Currency::SNT => Some("SNT".to_string()),
    Currency::SOON => Some("SOON".to_string()),
    Currency::SPHR => Some("SPHR".to_string()),
    Currency::SPR => Some("SPR".to_string()),
    Currency::SPRTS => Some("SPRTS".to_string()),
    Currency::SSD => Some("SSD".to_string()),
    Currency::START => Some("START".to_string()),
    Currency::STEEM => Some("STEEM".to_string()),
    Currency::STEPS => Some("STEPS".to_string()),
    Currency::STORJ => Some("STORJ".to_string()),
    Currency::STRAT => Some("STRAT".to_string()),
    Currency::STV => Some("STV".to_string()),
    Currency::SWIFT => Some("SWIFT".to_string()),
    Currency::SWING => Some("SWING".to_string()),
    Currency::SWT => Some("SWT".to_string()),
    Currency::SYNX => Some("SYNX".to_string()),
    Currency::SYS => Some("SYS".to_string()),
    Currency::TES => Some("TES".to_string()),
    Currency::THC => Some("THC".to_string()),
    Currency::TIME => Some("TIME".to_string()),
    Currency::TIT => Some("TIT".to_string()),
    Currency::TIX => Some("TIX".to_string()),
    Currency::TKN => Some("TKN".to_string()),
    Currency::TKS => Some("TKS".to_string()),
    Currency::TRI => Some("TRI".to_string()),
    Currency::TRIG => Some("TRIG".to_string()),
    Currency::TRK => Some("TRK".to_string()),
    Currency::TROLL => Some("TROLL".to_string()),
    Currency::TRST => Some("TRST".to_string()),
    Currency::TRUST => Some("TRUST".to_string()),
    Currency::TX => Some("TX".to_string()),
    Currency::U => Some("U".to_string()),
    Currency::UBQ => Some("UBQ".to_string()),
    Currency::UFO => Some("UFO".to_string()),
    Currency::UNB => Some("UNB".to_string()),
    Currency::UNIQ => Some("UNIQ".to_string()),
    Currency::UNIT => Some("UNIT".to_string()),
    Currency::UNO => Some("UNO".to_string()),
    Currency::USDT => Some("USDT".to_string()),
    Currency::UTC => Some("UTC".to_string()),
    Currency::VIA => Some("VIA".to_string()),
    Currency::VIOR => Some("VIOR".to_string()),
    Currency::VIRAL => Some("VIRAL".to_string()),
    Currency::VOX => Some("VOX".to_string()),
    Currency::VPN => Some("VPN".to_string()),
    Currency::VRC => Some("VRC".to_string()),
    Currency::VRM => Some("VRM".to_string()),
    Currency::VTC => Some("VTC".to_string()),
    Currency::VTR => Some("VTR".to_string()),
    Currency::WARP => Some("WARP".to_string()),
    Currency::WAVES => Some("WAVES".to_string()),
    Currency::WINGS => Some("WINGS".to_string()),
    Currency::XAUR => Some("XAUR".to_string()),
    Currency::XBB => Some("XBB".to_string()),
    Currency::XC => Some("XC".to_string()),
    Currency::XCO => Some("XCO".to_string()),
    Currency::XCP => Some("XCP".to_string()),
    Currency::XDN => Some("XDN".to_string()),
    Currency::XDQ => Some("XDQ".to_string()),
    Currency::XEL => Some("XEL".to_string()),
    Currency::XEM => Some("XEM".to_string()),
    Currency::XLM => Some("XLM".to_string()),
    Currency::XMG => Some("XMG".to_string()),
    Currency::XMR => Some("XMR".to_string()),
    Currency::XMY => Some("XMY".to_string()),
    Currency::XPY => Some("XPY".to_string()),
    Currency::XQN => Some("XQN".to_string()),
    Currency::XRP => Some("XRP".to_string()),
    Currency::XSEED => Some("XSEED".to_string()),
    Currency::XST => Some("XST".to_string()),
    Currency::XTC => Some("XTC".to_string()),
    Currency::XVC => Some("XVC".to_string()),
    Currency::XVG => Some("XVG".to_string()),
    Currency::XWC => Some("XWC".to_string()),
    Currency::XZC => Some("XZC".to_string()),
    Currency::YBC => Some("YBC".to_string()),
    Currency::ZCL => Some("ZCL".to_string()),
    Currency::ZEC => Some("ZEC".to_string()),
    Currency::ZEN => Some("ZEN".to_string()),
        _ => None,
    }
}
