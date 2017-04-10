#[cfg(test)]
mod bitstamp_tests {
    extern crate coinnect;
    use self::coinnect::bitstamp::utils;
    use self::coinnect::bitstamp::api::BitstampApi;

    use self::coinnect::exchange::ExchangeApi;
    use self::coinnect::pair::Pair;

    use std::collections::HashMap;

    #[test]
    fn build_url_should_return_the_a_url() {
        assert_eq!(utils::build_url("ticker", "btcusd"),
                   "https://www.bitstamp.net/api/v2/ticker/btcusd/");
    }
    #[test]
    fn build_url_should_return_the_url_for_transactions_for_btc_usd() {
        assert_eq!(utils::build_url("transactions", "btcusd"),
                   "https://www.bitstamp.net/api/v2/transactions/btcusd/");
    }

    #[test]
    fn can_get_real_bitstamp_tick() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.ticker(Pair::BTC_USD);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn ticker_should_have_the_correct_last() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.ticker(Pair::BTC_USD);
        assert!(result.unwrap().last_trade_price != 0.0);
    }
    #[test]
    fn ticker_should_have_the_correct_high() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.ticker(Pair::BTC_USD);
        assert!(result.unwrap().highest_bid != 0.0);
    }
    #[test]
    fn ticker_should_have_the_correct_low() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.ticker(Pair::BTC_USD);
        assert!(result.unwrap().lowest_ask != 0.0);
    }
    #[test]
    fn ticker_should_have_the_correct_volume() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.ticker(Pair::BTC_USD);
        assert!(result.unwrap().volume.unwrap() != 0.0);
    }

    #[test]
    fn should_return_an_order_book() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.return_order_book(Pair::BTC_USD);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn order_book_should_have_a_timestamp() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.return_order_book(Pair::BTC_USD);
        assert!(result.unwrap().contains_key("timestamp"));
    }
    #[test]
    fn order_book_should_have_bids() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.return_order_book(Pair::BTC_USD);
        assert!(result.unwrap().contains_key("bids"));
    }
    #[test]
    fn order_book_should_have_asks() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.return_order_book(Pair::BTC_USD);
        assert!(result.unwrap().contains_key("bids"));
    }

    #[test]
    fn order_book_should_have_asks_for_btcusd() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        assert!(api.return_order_book(Pair::BTC_USD).unwrap().contains_key("asks"));
    }
    #[test]
    fn order_book_should_have_asks_for_btceur() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        assert!(api.return_order_book(Pair::BTC_USD).unwrap().contains_key("asks"));
    }

    #[test]
    fn should_create_a_fixed_nonce_when_requested() {
        assert_eq!(utils::generate_nonce(Some("1".to_string())), "1");
    }
    #[test]
    fn should_create_a_nonce_bigger_than_2017() {
        assert!(utils::generate_nonce(None).parse::<i64>().unwrap() > 1483228800);
    }
    #[test]
    fn should_create_a_correct_signature() {
        let nonce = "1483228800".to_string();
        let customer_id = "123456".to_string();
        let api_key = "1234567890ABCDEF1234567890ABCDEF".to_string();
        let api_secret = "1234567890ABCDEF1234567890ABCDEF".to_string();
        let expected_signature = "7D7C4168D49CBC2620A45EF00EAA228C1287561F1C1F94172272E1231A8ADF6B"
            .to_string();
        assert_eq!(utils::build_signature(nonce, customer_id, api_key, api_secret),
                   expected_signature);
    }

    #[test]
    fn should_return_the_trade_history_for_btc_usd() {
        let params = HashMap::new();
        let mut api = BitstampApi::new(&params);
        let result = api.return_trade_history(Pair::BTC_USD);

        assert_eq!(result.is_ok(), false);
    }

    // IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    #[cfg_attr(not(feature = "bitstamp_private_tests"), ignore)]
    fn balance_should_have_usd_btc_fee() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let mut api = BitstampApi::new_from_file("account_bitstamp", path);
        let result = api.return_balances(Pair::BTC_USD).unwrap();
        let result_looking_for_usd = result.clone();
        let result_looking_for_btc = result.clone();
        let result_looking_for_fee = result.clone();

        assert!(result_looking_for_usd.contains_key("usd_balance"));
        assert!(result_looking_for_btc.contains_key("btc_balance"));
        assert!(result_looking_for_fee.contains_key("fee"));
    }
}
