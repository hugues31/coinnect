#[cfg(test)]
mod coinnect_tests {
    extern crate coinnect;

    use std::path::PathBuf;

    use self::coinnect::coinnect::Coinnect;
    use self::coinnect::error;
    use self::coinnect::exchange::{Exchange, ExchangeApi};
    use self::coinnect::pair::Pair;
    use self::coinnect::types::*;

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

    #[test]
    #[cfg_attr(not(feature = "kraken_private_tests"), ignore)]
    fn coinnect_can_add_order_from_kraken() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = Coinnect::new_from_file(Exchange::Kraken, "account_kraken", path);
        // following request should return an error since Kraken minimum order size is 0.01
        let orderinfo = api.add_order(OrderType::BuyLimit, Pair::BTC_EUR, 0.00001, Some(1000.58));

        assert_eq!(orderinfo.unwrap_err(), error::Error::InsufficientOrderSize)
    }
}
