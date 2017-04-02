#[cfg(test)]
mod coinnect_tests {
    extern crate coinnect;

    use self::coinnect::coinnect::Coinnect;
    use self::coinnect::exchange::{ Exchange, ExchangeApi };
    use self::coinnect::pair::Pair;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let api: Box<ExchangeApi> = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");

        assert_eq!(format!("{:?}", api), "BitstampApi { \
                                                last_request: 0, \
                                                api_key: \"bs_api_key\", \
                                                api_secret: \"bs_api_secret\", \
                                                customer_id: \"bs_cust_id\", \
                                                http_client: Client { \
                                                    redirect_policy: FollowAll, \
                                                    read_timeout: None, \
                                                    write_timeout: None, \
                                                    proxy: None \
                                                } \
                                            }");
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
        let mut api = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let ticker = api.return_ticker(Pair::BtcUsd);

        assert!( ticker.is_some() );
    }
    #[test]
    fn coinnect_ticker_from_bitstamp_should_have_the_correct_last() {
        let mut api = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let ticker = api.return_ticker(Pair::BtcUsd);

        assert!( ticker.unwrap().contains_key("last") );
    }

    #[test]
    fn coinnect_should_return_an_order_book_from_bitstamp() {
        let mut api = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let order_book = api.return_order_book(Pair::BtcUsd);
        assert!( order_book.is_some() );
    }
    #[test]
    fn order_book_should_have_bids() {
        let mut api = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let result = api.return_order_book(Pair::BtcUsd);
        assert!( result.unwrap().contains_key("bids") );
    }

    #[test]
    fn public_query_should_be_able_to_return_the_trade_history_for_btc_usd_from_bitstamp() {
        use std::collections::HashMap;

        let mut api = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");

        let mut params = HashMap::new();
        params.insert("pair", "btcusd");
        params.insert("method", "transactions");
        let result = api.public_query(&params);

        assert_eq!( result.is_some(), false );
    }

    #[test]
    fn should_return_the_trade_history_for_btc_usd_from_bitstamp() {
        let mut api = Coinnect::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let result = api.return_trade_history(Pair::BtcUsd);

        assert_eq!( result.is_some(), false );
    }

    // IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    fn balance_should_have_usd_btc_fee() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let mut api = Coinnect::new_from_file(Exchange::Bitstamp , path);
        let result = api.return_balances(Pair::BtcUsd).unwrap();
        let result_looking_for_usd = result.clone();
        let result_looking_for_btc = result.clone();
        let result_looking_for_fee = result.clone();

        assert!(result_looking_for_usd.contains_key("usd_balance"));
        assert!(result_looking_for_btc.contains_key("btc_balance"));
        assert!(result_looking_for_fee.contains_key("fee"));
    }
}