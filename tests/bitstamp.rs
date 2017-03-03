#[cfg(test)]
mod bitstamp_tests {
    extern crate coinnect;
    use self::coinnect::bitstamp::BitstampApi;

    #[test]
    fn build_url_should_return_the_a_url() {
        assert_eq!(
            BitstampApi::build_url("ticker", "btcusd"),
            "https://www.bitstamp.net/api/v2/ticker/btcusd/");
    }

    #[test]
    fn can_get_real_bitstamp_tick() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn ticker_should_have_the_correct_last() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("last"));
    }
    #[test]
    fn ticker_should_have_the_correct_high() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("high"));
    }
    #[test]
    fn ticker_should_have_the_correct_low() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("low"));
    }
    #[test]
    fn ticker_should_have_the_correct_vwap() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("vwap"));
    }
    #[test]
    fn ticker_should_have_the_correct_volume() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("volume"));
    }
    #[test]
    fn ticker_should_have_the_correct_bid() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("bid"));
    }
    #[test]
    fn ticker_should_have_the_correct_ask() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("ask"));
    }
    #[test]
    fn ticker_should_have_the_correct_timestamp() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("timestamp"));
    }
    #[test]
    fn ticker_should_have_the_correct_open() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_ticker();
        assert!(result.unwrap().contains_key("open"));
    }

    #[test]
    fn should_return_an_order_book() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_order_book("btcusd");
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn order_book_should_have_a_timestamp() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_order_book("btcusd");
        assert!(result.unwrap().contains_key("timestamp"));
    }
    #[test]
    fn order_book_should_have_bids() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_order_book("btcusd");
        assert!(result.unwrap().contains_key("bids"));
    }
    #[test]
    fn order_book_should_have_asks() {
        let mut api = BitstampApi::new("", "");
        let result = api.return_order_book("btcusd");
        assert!(result.unwrap().contains_key("bids"));
    }

    #[test]
    fn order_book_should_have_asks_for_btcusd() {
        let mut api = BitstampApi::new("", "");
        assert!(api.return_order_book("btcusd").unwrap().contains_key("asks"));
    }
    #[test]
    fn order_book_should_have_asks_for_btceur() {
        let mut api = BitstampApi::new("", "");
        assert!(api.return_order_book("btceur").unwrap().contains_key("asks"));
    }

    #[test]
    fn should_create_a_fixed_nonce_when_requested() {
        assert_eq!(BitstampApi::generate_nonce(Some("1".to_string())), "1");
    }
    #[test]
    fn should_create_a_nonce_bigger_than_2017() {
        assert!(BitstampApi::generate_nonce(None).parse::<i64>().unwrap() > 1483228800);
    }
    #[test]
    fn should_create_a_correct_signature() {
        assert_eq!(
            BitstampApi::build_signature("123456", "1234567890ABCDEF1234567890ABCDEF", "1234567890ABCDEF1234567890ABCDEF", "1".to_string()),
            "0421E65BB519A045016A2E4B2F71F50579AA8AD3F6059109C61A3064ACF45712");
    }
}