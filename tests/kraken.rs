#[cfg(test)]
mod kraken_tests {
    extern crate coinnect;

    use self::coinnect_rt::kraken::{KrakenApi, KrakenCreds};
    use self::coinnect_rt::bitstamp::BitstampCreds;

    #[test]
    fn fail_with_invalid_creds() {
        let creds = BitstampCreds::new("", "", "", "");
        let res = KrakenApi::new(creds);
        assert_eq!(res.unwrap_err().to_string(),
                   "Invalid config: \nExpected: Kraken\nFind: Bitstamp");
    }

    /// IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    #[cfg_attr(not(feature = "kraken_private_tests"), ignore)]
    fn balance_should_return_a_result() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let creds = KrakenCreds::new_from_file("account_kraken", path).unwrap();
        let mut api = KrakenApi::new(creds).unwrap();

        let result = api.get_account_balance().unwrap();

        assert!(result.contains_key("result"))
    }
}
