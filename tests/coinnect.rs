#[cfg(test)]
mod coinnect_tests {
    extern crate coinnect;

    use self::coinnect::coinnect::Coinnect;
    use self::coinnect::exchange::Exchange;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let api = Coinnect::new(Exchange::Bitstamp, "", "", "");
        assert_eq!(api, Exchange::Bitstamp);
    }
    #[test]
    fn can_create_new_api_connection_to_kraken() {
        let api = Coinnect::new(Exchange::Kraken, "", "", "");
        assert_eq!(api, Exchange::Kraken);
    }
    #[test]
    fn can_create_new_api_connection_to_poloniex() {
        let api = Coinnect::new(Exchange::Poloniex, "", "", "");
        assert_eq!(api, Exchange::Poloniex);
    }
}