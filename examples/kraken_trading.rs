// This example shows how to implement a simple trading strategy.
// We are looking for the pair with the highest rise in price over the last 24 hours and we
// add a buy order to buy it.

// Please do NOT run this example with your real account unless you know what you're doing.

extern crate coinnect;

use std::path::PathBuf;

use coinnect::kraken::KrakenApi;

fn main() {
    // We create a KrakenApi by loading a json file containing API configuration
    // (see documentation for more info)
    let path = PathBuf::from("keys_real.json");
    let mut my_api = KrakenApi::new_from_file("account_kraken", path);

    // First, get the list of all pair we can trade with EUR€ as quote
    let pairs_request = my_api.get_tradable_asset_pairs("", "").unwrap();
    let list_all_pairs = pairs_request.get("result").unwrap().as_object().unwrap();

    let mut list_pairs_eur = Vec::new();

    for pair in list_all_pairs {
        // The map structure is explained in documentation
        let quote = pair.1.as_object().unwrap().get("quote").unwrap().as_str().unwrap();
        if quote == "ZEUR" {
            let name = pair.0;
            list_pairs_eur.push(name);
        }
    }

    println!("List {:?}", list_pairs_eur);

    // Now that we have the pairs, we choose the one with the highest price variation.
    // We query the ticker to get the opening and closing price over the last 24 hours for each
    // pairs in list_pairs_eur. KrakenApi has a blocking timer which prevents from ban if you
    // make rapid succession of requests (inside a for loop for example). Here, the ticker function
    // can take a list of pairs as parameter, so 1 request should suffice.

    // Convert Vec into comma separated values String
    let eur_pairs = format!("{:?}", list_pairs_eur);
    let eur_pairs = eur_pairs.replace("\"", "").replace("[", "").replace("]", "").replace(" ", "");


    // Get ticker
    let ticker_request = my_api.get_ticker_information(&eur_pairs).unwrap();
    let list_ticker = ticker_request.get("result").unwrap().as_object().unwrap();

    let mut pair_to_buy = "";
    let mut pair_price_var = 0.0;
    let mut current_price = 0.0;

    for pair in list_ticker {
        let name = pair.0;

        // WARNING: Kraken uses quotes to encapsulate floating value
        let pair_info = pair.1.as_object().unwrap();
        let open_price = pair_info.get("o").unwrap().as_str().unwrap().parse::<f64>().unwrap();
        let close_price_array = pair_info.get("c").unwrap().as_array().unwrap();
        let close_price = close_price_array[0].as_str().unwrap().parse::<f64>().unwrap();

        let price_var = (close_price / open_price - 1.0) * 100.0;

        if price_var > pair_price_var {
            pair_price_var = price_var;
            pair_to_buy = name;
            current_price = close_price;
        }
    }

    println!("{} has the highest price variation ({:.2}%).",
             pair_to_buy,
             pair_price_var);

    // Add a buy limit order for an amount of 100€ for a price of: current_price - 2%
    let buying_price = current_price - (current_price * 2.0 / 100.0);
    let volume = 100.0 / buying_price;

    // Specify optional parameters with an empty str ("")
    // See documentation for more informations
    my_api.add_standard_order(pair_to_buy,                  // name of the pair
                              "buy",                        // type : buy/sell
                              "limit",                      // order type : market/limit/...
                              &buying_price.to_string(),    // price 1
                              "",                           // price 2
                              &volume.to_string(),          // volume
                              "",                           // leverage
                              "",                           // oflags (see doc)
                              "",                           // starttm
                              "",                           // expiretm
                              "",                           // userref
                              "");                          // validate
    // In a real case example, you should check if any error occurs.
}
