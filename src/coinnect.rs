use exchange::Exchange;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Coinnect {
    BitstampApi
}

impl Coinnect {
    /// Create a new CoinnectApi by providing an API key & API secret
    pub fn new(exchange: Exchange, customer_id: &str, api_key: &str, api_secret: &str) -> Exchange {
        println!("customer_id: {}, api_key: {}, api_secret: {}", customer_id, api_key, api_secret);
        match exchange {
            Exchange::Bitstamp => Exchange::Bitstamp,
            Exchange::Kraken => Exchange::Kraken,
            Exchange::Poloniex => Exchange::Poloniex,
        }
    }
}