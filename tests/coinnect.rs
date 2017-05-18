#[cfg(test)]
mod coinnect_tests {
    extern crate coinnect;

    use self::coinnect::coinnect::Coinnect;
    use self::coinnect::exchange::{Exchange, ExchangeApi};
    use self::coinnect::pair::Pair;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let api: Box<ExchangeApi> = Coinnect::new(Exchange::Bitstamp,
                                                  "bs_api_key",
                                                  "bs_api_secret",
                                                  Some("bs_cust_id"));

        assert_eq!(format!("{:?}", api),
                   "BitstampApi { last_request: 0, api_key: \"bs_api_key\", api_secret: \
                    \"bs_api_secret\", customer_id: \"bs_cust_id\", http_client: Client { \
                    redirect_policy: FollowAll, read_timeout: None, write_timeout: None, proxy: \
                    None } }");
    }
    #[test]
    fn can_create_new_api_connection_to_kraken() {
        //        let api = Coinnect::new(Exchange::Kraken, "", "", "");
        //        assert_eq!(api, Exchange::Kraken);
    }
    #[test]
    fn can_create_new_api_connection_to_poloniex() {
        //        let api = Coinnect::new(Exchange::Poloniex, "", "", "");
        //        assert_eq!(api, Exchange::Poloniex);
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_bitstamp() {
        let mut api = Coinnect::new(Exchange::Bitstamp,
                                    "bs_api_key",
                                    "bs_api_secret",
                                    Some("bs_cust_id"));
        let ticker = api.ticker(Pair::BTC_USD);

        assert_ne!(ticker.unwrap().last_trade_price, 0.0);
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_kraken() {
        let mut api = Coinnect::new(Exchange::Kraken, "api_key", "api_secret", None);
        let ticker = api.ticker(Pair::BTC_EUR);

        assert_ne!(ticker.unwrap().last_trade_price, 0.0);
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_poloniex() {
        let mut api = Coinnect::new(Exchange::Poloniex, "api_key", "api_secret", None);
        let ticker = api.ticker(Pair::BTC_ETH);

        assert_ne!(ticker.unwrap().last_trade_price, 0.0);
    }

    #[test]
    fn coinnect_can_get_an_orderbook_from_kraken() {
        let mut api = Coinnect::new(Exchange::Kraken, "api_key", "api_secret", None);
        let orderbook = api.orderbook(Pair::BTC_EUR);

        assert_ne!(orderbook.unwrap().avg_price().unwrap(), 0.0)
    }

    #[test]
    fn coinnect_can_get_an_orderbook_from_poloniex() {
        let mut api = Coinnect::new(Exchange::Poloniex, "api_key", "api_secret", None);
        let orderbook = api.orderbook(Pair::BTC_ETH);

        assert_ne!(orderbook.unwrap().avg_price().unwrap(), 0.0)
    }
}
