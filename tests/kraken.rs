#[cfg(test)]
mod kraken_tests {
    extern crate coinnect;

    use self::coinnect::kraken::api::KrakenApi;

    /// IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    fn balance_should_return_a_result() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let mut api = KrakenApi::new_from_file("account_kraken", path);
        let result = api.get_account_balance();

        assert!(result.unwrap().contains_key("result"));
    }
}
