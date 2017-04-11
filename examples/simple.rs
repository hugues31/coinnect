// This example shows how to connect to your Poloniex account and perform simple operations

extern crate coinnect;

use coinnect::poloniex::api::PoloniexApi;

fn main() {
    // We create a PoloniexApi by providing API key/secret
    // You can give an empty String if you only use public methods
    let mut my_api = PoloniexApi::new("api_key", "api_secret");

    // Let's look at the ticker!
    let list_coins = my_api.return_ticker().unwrap();

    for coin in list_coins {
        // please visit Poloniex API documentation to know how the data is returned
        // or look at the coinnect documentation
        let name = coin.0;
        let price = coin.1.as_object().unwrap().get("last").unwrap().as_str().unwrap();

        println!("Coin {} has price : {}", name, price);
    }
}
