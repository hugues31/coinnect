#[cfg(test)]
mod poloniex_tests {
    extern crate coinnect;

    use self::coinnect_rt::poloniex::{PoloniexApi, PoloniexCreds};
    use self::coinnect_rt::bitstamp::BitstampCreds;

    #[test]
    fn fail_with_invalid_creds() {
        let creds = BitstampCreds::new("", "", "", "");
        let res = PoloniexApi::new(creds);
        assert_eq!(res.unwrap_err().to_string(),
                   "Invalid config: \nExpected: Poloniex\nFind: Bitstamp");
    }

    /// IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    #[cfg_attr(not(feature = "poloniex_private_tests"), ignore)]
    fn balance_has_btc_key() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let creds = PoloniexCreds::new_from_file("account_poloniex", path).unwrap();
        let mut api = PoloniexApi::new(creds).unwrap();
        let result = api.return_balances();

        assert!(result.unwrap().contains_key("BTC"));
    }
}
