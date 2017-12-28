#[cfg(test)]
mod gdax_tests {
    extern crate coinnect;
    extern crate bigdecimal;

    use self::bigdecimal::BigDecimal;
    use std::str::FromStr;

    use self::coinnect::gdax::utils;
    use self::coinnect::gdax::{GdaxApi, GdaxCreds};
    use self::coinnect::bitstamp::BitstampCreds;

    use self::coinnect::exchange::ExchangeApi;
    use self::coinnect::types::Pair;

    #[test]
    fn build_url_should_return_the_a_url() {
        assert_eq!(utils::build_url("ticker", "btc-usd"),
                   "https://api.gdax.com/products/btc-usd/ticker");
    }
    #[test]
    fn build_url_should_return_the_url_for_transactions_for_btc_usd() {
        assert_eq!(utils::build_url("transactions", "btc-usd"),
                   "https://api.gdax.com/accounts/btc-usd/ledger");
    }

    #[test]
    fn fail_with_invalid_creds() {
        let creds = BitstampCreds::new("", "", "", "");
        let res = GdaxApi::new(creds);
        assert_eq!(res.unwrap_err().to_string(),
                   "Invalid config: \nExpected: Gdax\nFind: Bitstamp");
    }


    #[test]
    fn can_get_real_gdax_tick() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        api.ticker(Pair::BTC_USD).unwrap();
    }

    #[test]
    fn ticker_should_have_the_correct_last() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        let result = api.ticker(Pair::BTC_USD);
        assert_ne!(result.unwrap().last_trade_price,
                   BigDecimal::from_str("0.0").unwrap());
    }
    #[test]
    fn ticker_should_have_the_correct_high() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        let result = api.ticker(Pair::BTC_USD);
        assert_ne!(result.unwrap().highest_bid,
                   BigDecimal::from_str("0.0").unwrap());
    }
    #[test]
    fn ticker_should_have_the_correct_low() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        let result = api.ticker(Pair::BTC_USD);
        assert_ne!(result.unwrap().lowest_ask,
                   BigDecimal::from_str("0.0").unwrap());
    }
    #[test]
    fn ticker_should_have_the_correct_volume() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        let result = api.ticker(Pair::BTC_USD);
        assert_ne!(result.unwrap().volume.unwrap(),
                   BigDecimal::from_str("0.0").unwrap());
    }

    #[test]
    fn should_return_an_order_book() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        let result = api.return_order_book(Pair::BTC_USD);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn order_book_should_have_a_timestamp() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        let result = api.return_order_book(Pair::BTC_USD);
        assert!(result.unwrap().contains_key("sequence"));
    }
    #[test]
    fn order_book_should_have_asks_for_btcusd() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        assert!(api.return_order_book(Pair::BTC_USD)
                    .unwrap()
                    .contains_key("asks"));
    }
    #[test]
    fn order_book_should_have_asks_for_btceur() {
        let creds = GdaxCreds::new("", "", "", "");
        let mut api = GdaxApi::new(creds).unwrap();
        assert!(api.return_order_book(Pair::BTC_USD)
                    .unwrap()
                    .contains_key("asks"));
    }

    #[test]
    fn should_create_a_fixed_nonce_when_requested() {
        assert_eq!(utils::generate_nonce(Some("1".to_string())), "1");
    }
    #[test]
    fn should_create_a_nonce_bigger_than_2017() {
        assert!(utils::generate_nonce(None).parse::<i64>().unwrap() > 1483228800);
    }
//    #[test]
//    fn should_create_a_correct_signature() {
//        let nonce = "1483228800";
//        let passphrase = "123456";
//        let api_key = "1234567890ABCDEF1234567890ABCDEF";
//        let api_secret = "1234567890ABCDEF1234567890ABCDEF";
//        let expected_signature = "7D7C4168D49CBC2620A45EF00EAA228C1287561F1C1F94172272E1231A8ADF6B"
//            .to_string();
//        assert_eq!(utils::build_signature(nonce, passphrase, api_key, api_secret).unwrap(),
//                   expected_signature);
//    }
//
//    #[test]
//    fn should_return_the_trade_history_for_btc_usd() {
//        let creds = GdaxCreds::new("", "", "", "");
//        let mut api = GdaxApi::new(creds).unwrap();
//        let result = api.return_trade_history(Pair::BTC_USD);
//
//        assert_eq!(result.is_ok(), false);
//    }
//
//    // IMPORTANT: Real keys are needed in order to retrieve the balance
//    #[test]
////    #[cfg_attr(not(feature = "gdax_private_tests"), ignore)]
//    fn balance_should_have_usd_and_btc_balance() {
//        use std::path::PathBuf;
//        let path = PathBuf::from("./keys_real.json");
//        let creds = GdaxCreds::new_from_file("account_gdax", path).unwrap();
//        let mut api = GdaxApi::new(creds).unwrap();
//        let result = api.return_balances().unwrap();
//        let result_looking_for_usd = result.clone();
//        let result_looking_for_btc = result.clone();
//
//        assert!(result_looking_for_usd.contains_key("usd_balance"));
//        assert!(result_looking_for_btc.contains_key("btc_balance"));
//    }
}
