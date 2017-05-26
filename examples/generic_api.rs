// This example shows how to use the generic API provided by Coinnect.
// This method is useful if you have to iterate throught multiple accounts of
// different exchanges and perform the same operation (such as get the current account's balance)
// You can also use the Coinnect generic API if you want a better error handling since all methods
// return Result<_, Error>.

extern crate coinnect;

use coinnect::coinnect::Coinnect;
use coinnect::kraken::KrakenCreds;
use coinnect::exchange::Exchange::*;
use coinnect::pair::Pair::*;

fn main() {
    // We create a Coinnect Generic API
    // Since Kraken does not need customer_id field, we set it to None
    let my_creds = KrakenCreds::new("my_optionnal_name", "api_key", "api_secret");
    let mut my_api = Coinnect::new(Kraken, my_creds).unwrap();
    let ticker = my_api.ticker(ETC_BTC);

    println!("ETC_BTC last trade price is {}.",
             ticker.unwrap().last_trade_price);
}
