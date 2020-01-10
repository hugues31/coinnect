#[cfg(test)]
mod bittrex_tests {
    extern crate coinnect;

    use self::coinnect_rt::bittrex::{BittrexApi, BittrexCreds};

    #[test]
    fn get_markets_should_return_a_result() {
        let creds = BittrexCreds::new("bittrex", "", "");
        let mut api = BittrexApi::new(creds).unwrap();

        let result = api.get_markets().unwrap();

        assert!(result.contains_key("success"))
    }

    #[test]
    fn get_ticker_should_return_a_ticker() {
        let creds = BittrexCreds::new("bittrex", "", "");
        let mut api = BittrexApi::new(creds).unwrap();

        let result = api.get_ticker("BTC-LTC").unwrap();

        let last_price = result.get("result").unwrap().as_object().unwrap().get("Last");
        assert!(last_price.is_some())
    }

    /// IMPORTANT: Real keys are needed in order to retrieve the balances
    #[test]
    #[cfg_attr(not(feature = "bittrex_private_tests"), ignore)]
    async fn balances_should_return_a_result() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let creds = BittrexCreds::new_from_file("account_bittrex", path).unwrap();
        let mut api = BittrexApi::new(creds).unwrap();

        let result = api.get_balances().unwrap();

        assert!(result.get("result").is_some())
    }
}
