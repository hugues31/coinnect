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

}