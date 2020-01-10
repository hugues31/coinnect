#[cfg(test)]
mod coinnect_tests {
    extern crate coinnect;
    extern crate bigdecimal;

    use self::bigdecimal::BigDecimal;
    use std::str::FromStr;
    use std::path::PathBuf;

    use self::coinnect_rt::coinnect::Coinnect;
    use self::coinnect_rt::exchange::{Exchange, ExchangeApi};
    use self::coinnect_rt::kraken::KrakenCreds;
    use self::coinnect_rt::bitstamp::BitstampCreds;
    use self::coinnect_rt::poloniex::PoloniexCreds;
    use self::coinnect_rt::bittrex::BittrexCreds;
    use self::coinnect_rt::error::*;
    use self::coinnect_rt::types::*;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let creds = BitstampCreds::new("test", "bs_api_key", "bs_api_secret", "bs_cust_id");
        let api: Box<ExchangeApi> = coinnect_rt::new(Exchange::Bitstamp, creds).unwrap();

        assert_eq!(format!("{:?}", api),
                   "BitstampApi { last_request: 0, api_key: \"bs_api_key\", api_secret: \
        \"bs_api_secret\", customer_id: \"bs_cust_id\", http_client: Client { \
                    redirect_policy: FollowAll, read_timeout: None, write_timeout: None, proxy: \
                    None }, burst: false }");
    }
    #[test]
    fn can_create_new_api_connection_to_kraken() {
        //        let api = coinnect_rt::new(Exchange::Kraken, "", "", "");
        //        assert_eq!(api, Exchange::Kraken);
    }
    #[test]
    fn can_create_new_api_connection_to_poloniex() {
        //        let api = coinnect_rt::new(Exchange::Poloniex, "", "", "");
        //        assert_eq!(api, Exchange::Poloniex);
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_bitstamp() {
        let creds = BitstampCreds::new("test", "bs_api_key", "bs_api_secret", "bs_cust_id");
        let mut api = coinnect_rt::new(Exchange::Bitstamp, creds).unwrap();
        let ticker = api.ticker(Pair::BTC_USD);

        assert_ne!(ticker.unwrap().last_trade_price, BigDecimal::from_str("0.0").unwrap());
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_kraken() {
        let creds = KrakenCreds::new("test", "api_key", "api_secret");
        let mut api = coinnect_rt::new(Exchange::Kraken, creds).unwrap();
        let ticker = api.ticker(Pair::BTC_EUR);

        assert_ne!(ticker.unwrap().last_trade_price, BigDecimal::from_str("0.0").unwrap());
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_poloniex() {
        let creds = PoloniexCreds::new("test", "api_key", "api_secret");
        let mut api = coinnect_rt::new(Exchange::Poloniex, creds).unwrap();
        let ticker = api.ticker(Pair::ETH_BTC);

        assert_ne!(ticker.unwrap().last_trade_price, BigDecimal::from_str("0.0").unwrap());
    }

    #[test]
    fn coinnect_can_get_a_ticker_from_bittrex() {
        let creds = BittrexCreds::new("test", "api_key", "api_secret");
        let mut api = coinnect_rt::new(Exchange::Bittrex, creds).unwrap();
        let ticker = api.ticker(Pair::ETH_BTC);

        assert_ne!(ticker.unwrap().last_trade_price, BigDecimal::from_str("0.0").unwrap());
    }

    #[test]
    fn coinnect_can_get_an_orderbook_from_bitstamp() {
        let creds = BitstampCreds::new("test", "api_key", "api_secret", "customer_id");
        let mut api = coinnect_rt::new(Exchange::Bitstamp, creds).unwrap();
        let orderbook = api.orderbook(Pair::BTC_EUR);

        assert_ne!(orderbook.unwrap().avg_price().unwrap(), BigDecimal::from_str("0.0").unwrap())
    }

    #[test]
    fn coinnect_can_get_an_orderbook_from_kraken() {
        let creds = KrakenCreds::new("test", "api_key", "api_secret");
        let mut api = coinnect_rt::new(Exchange::Kraken, creds).unwrap();
        let orderbook = api.orderbook(Pair::BTC_EUR);

        assert_ne!(orderbook.unwrap().avg_price().unwrap(), BigDecimal::from_str("0.0").unwrap())
    }

    #[test]
    fn coinnect_can_get_an_orderbook_from_poloniex() {
        let creds = PoloniexCreds::new("test", "api_key", "api_secret");
        let mut api = coinnect_rt::new(Exchange::Poloniex, creds).unwrap();
        let orderbook = api.orderbook(Pair::ETH_BTC);

        assert_ne!(orderbook.unwrap().avg_price().unwrap(), BigDecimal::from_str("0.0").unwrap())
    }

    #[test]
    fn coinnect_can_get_an_orderbook_from_bittrex() {
        let creds = BittrexCreds::new("test", "api_key", "api_secret");
        let mut api = coinnect_rt::new(Exchange::Bittrex, creds).unwrap();
        let orderbook = api.orderbook(Pair::ETH_BTC);

        assert_ne!(orderbook.unwrap().avg_price().unwrap(), BigDecimal::from_str("0.0").unwrap())
    }

    #[test]
    #[cfg_attr(not(feature = "bitstamp_private_tests"), ignore)]
    fn coinnect_can_get_the_balances_from_bitstamp() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = coinnect_rt::new_from_file(Exchange::Bitstamp, "account_bitstamp", path)
            .unwrap();
        let balances: Balances = api.balances().unwrap();

        assert!(balances.len() > 0)
    }

    #[test]
    #[cfg_attr(not(feature = "poloniex_private_tests"), ignore)]
    fn coinnect_can_get_the_balances_from_poloniex() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = coinnect_rt::new_from_file(Exchange::Poloniex, "account_poloniex", path)
            .unwrap();
        let balances: Balances = api.balances().unwrap();

        assert!(balances.len() > 0)
    }

    #[test]
    #[cfg_attr(not(feature = "bitstamp_private_tests"), ignore)]
    fn coinnect_can_get_at_least_a_positive_balance_from_bitstamp() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = coinnect_rt::new_from_file(Exchange::Bitstamp, "account_bitstamp", path)
            .unwrap();
        let balances: Balances = api.balances().unwrap();

        assert!(balances.get(&Currency::BTC).unwrap() >= &BigDecimal::from_str("0.0").unwrap());
        assert!(balances.get(&Currency::EUR).unwrap() >= &BigDecimal::from_str("0.0").unwrap());
        assert!(balances.get(&Currency::USD).unwrap() >= &BigDecimal::from_str("0.0").unwrap());
        assert!(balances.get(&Currency::XRP).unwrap() >= &BigDecimal::from_str("0.0").unwrap())
    }

    #[test]
    #[cfg_attr(not(feature = "kraken_private_tests"), ignore)]
    fn coinnect_can_get_the_balances_from_kraken() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = coinnect_rt::new_from_file(Exchange::Kraken, "account_kraken", path)
            .unwrap();
        let balances: Balances = api.balances().unwrap();

        assert!(balances.len() > 0);
        assert!(balances.get(&Currency::BTC).unwrap() >= &BigDecimal::from_str("0.0").unwrap())
    }

    #[test]
    #[cfg_attr(not(feature = "poloniex_private_tests"), ignore)]
    fn coinnect_can_get_at_least_a_positive_balance_from_poloniex() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = coinnect_rt::new_from_file(Exchange::Poloniex, "account_poloniex", path)
            .unwrap();
        let balances: Balances = api.balances().unwrap();
        let mut is_positive = false;
        for (_, balance) in &balances {
            if balance > &BigDecimal::from_str("0.0").unwrap() {
                is_positive = true;
                break;
            }
        }
        assert!(is_positive)
    }

    #[test]
    #[cfg_attr(not(feature = "bittrex_private_tests"), ignore)]
    fn coinnect_can_get_at_least_a_positive_balance_from_bittrex() {
        let path = PathBuf::from("./keys_real.json");
        let mut api = coinnect_rt::new_from_file(Exchange::Bittrex, "account_bittrex", path)
            .unwrap();
        let balances: Balances = api.balances().unwrap();
        let mut is_positive = false;
        for (_, balance) in &balances {
            if balance >= &BigDecimal::from_str("0.0").unwrap() {
                is_positive = true;
                break;
            }
        }
        assert!(is_positive)
    }

    #[test]
    #[cfg_attr(not(feature = "kraken_private_tests"), ignore)]
    fn coinnect_can_add_order_from_kraken() {
        let path = PathBuf::from("./keys_real.json");
        let creds = KrakenCreds::new_from_file("account_kraken", path).unwrap();
        let mut api = coinnect_rt::new(Exchange::Kraken, creds).unwrap();
        // following request should return an error since Kraken minimum order size is BigDecimal::from_str("0.01")?
        let orderinfo = api.add_order(OrderType::BuyLimit,
                                      Pair::BTC_EUR,
                                      BigDecimal::from_str("0.00001").unwrap(),
                                      Some(BigDecimal::from_str("1000.58").unwrap()));

        assert_eq!(orderinfo.unwrap_err().to_string(),
                   ErrorKind::InsufficientOrderSize.to_string())
    }

    #[test]
    #[cfg_attr(not(feature = "poloniex_private_tests"), ignore)]
    fn coinnect_can_add_order_from_poloniex() {
        let path = PathBuf::from("./keys_real.json");
        let creds = PoloniexCreds::new_from_file("account_poloniex", path).unwrap();
        let mut api = coinnect_rt::new(Exchange::Poloniex, creds).unwrap();
        // following request should return an error
        let orderinfo = api.add_order(OrderType::BuyLimit,
                                      Pair::ETH_BTC,
                                      BigDecimal::from_str("0.00001").unwrap(),
                                      Some(BigDecimal::from_str("1000.58").unwrap()));

        assert_eq!(orderinfo.unwrap_err().to_string(),
                   ErrorKind::InsufficientOrderSize.to_string())
    }

    #[test]
    #[cfg_attr(not(feature = "bitstamp_private_tests"), ignore)]
    fn coinnect_can_add_order_from_bitstamp() {
        let path = PathBuf::from("./keys_real.json");
        let creds = BitstampCreds::new_from_file("account_bitstamp", path).unwrap();
        let mut api = coinnect_rt::new(Exchange::Bitstamp, creds).unwrap();
        // following request should return an error
        let orderinfo = api.add_order(OrderType::BuyLimit,
                                      Pair::EUR_USD,
                                      BigDecimal::from_str("0.00001").unwrap(),
                                      Some(BigDecimal::from_str("1000.58").unwrap()));

        assert_eq!(orderinfo.unwrap_err().to_string(),
                   ErrorKind::InsufficientOrderSize.to_string())
    }

    #[test]
    #[cfg_attr(not(feature = "bittrex_private_tests"), ignore)]
    fn coinnect_can_add_order_from_bittrex() {
        let path = PathBuf::from("./keys_real.json");
        let creds = BittrexCreds::new_from_file("account_bittrex", path).unwrap();
        let mut api = coinnect_rt::new(Exchange::Bittrex, creds).unwrap();
        // following request should return an error
        let orderinfo = api.add_order(OrderType::BuyLimit,
                                      Pair::ETH_BTC,
                                      BigDecimal::from_str("0.000000000001").unwrap(),
                                      Some(BigDecimal::from_str("1000.58").unwrap()));

        assert_eq!(orderinfo.unwrap_err().to_string(),
                   ErrorKind::InsufficientOrderSize.to_string())
    }
}
