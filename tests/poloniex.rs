#[cfg(test)]
mod poloniex_tests {
    extern crate coinnect;

    use self::coinnect::poloniex::api::PoloniexApi;

    /// IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    #[cfg_attr(not(feature = "poloniex_private_tests"), ignore)]
    fn balance_has_btc_key() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let mut api = PoloniexApi::new_from_file("account_poloniex", path).unwrap();
        let result = api.return_balances();

        assert!(result.unwrap().contains_key("BTC"));
    }
}
