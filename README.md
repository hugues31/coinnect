![Coinnect](https://raw.githubusercontent.com/hugues31/coinnect/master/coinnect.png)
===========
[![crates.io](https://img.shields.io/crates/v/coinnect.svg)](https://crates.io/crates/coinnect)
[![Downloads from crates.io](https://img.shields.io/crates/d/coinnect.svg)](https://crates.io/crates/coinnect)
[![Build Status](https://travis-ci.org/hugues31/coinnect.svg?branch=master)](https://travis-ci.org/hugues31/coinnect)
[![doc.rs](https://docs.rs/coinnect/badge.svg)](https://docs.rs/coinnect/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


Coinnect is a Rust library aiming to provide a complete access to REST APIs for
various crypto-currencies exchanges (see below for a list of supported
exchanges).
All methods consume HTTPS api. The purpose of this crate is not to stream data
(you should use websocket/FIX in that case).

You basically have 2 choices to retrieve data : use the raw API provided by the
platform you target or use the generic Coinnect API, which is more user-friendly
and safe. Ideally, use the raw API when the Coinnect API could not retrieve the
data/perform the action you want.

**WARNING:**  This library is highly experimental at the moment. Please do not
invest what you can't afford to lose. This is a personal project, I cannot be
held responsible for the library malfunction, which can lead to a loss of money.

*The project is licensed under the terms of the MIT License.*

### Exchanges support:
| Exchange | Raw API supported | Generic API supported | Note |
|:--------:|:-----------------:|:---------------------:|:----:|
| Bitstamp | X | X | Not every method is implemented for now.|
| Kraken   | X | X | - |
| Poloniex | X | X | - |
| Bittrex  | X | - | Only public methods are available, more to come |

Generic API supports:
 - Ticker
 - Orderbook
 - Balances
 - Add a new order
 - ... more to come!

Feel free to make a PR to add support to your favorite exchange ;)

### Documentation

- [Master](https://docs.rs/coinnect/)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
coinnect = "0.5"
```

and this to your crate root:

```rust
extern crate coinnect;
```

For optional parameters, most methods require an empty `str` (`""`) or
`Option` (`None`) if you don't want to specify them.

Since 0.2, you have access to a generic API to communicate across exchanges in
the same way. Note that this functionality is under active development.
For more informations, look at ExchangeApi trait doc.

## Example

The example below shows you how to connect to Poloniex

```rust
extern crate coinnect;

use coinnect::poloniex::api::PoloniexApi;
use coinnect::poloniex::credentials::PoloniexCreds;

fn main() {
    // We create a PoloniexApi by providing API key/secret
    // You can give an empty str if you only use public methods
    let creds = PoloniexCreds::new("my_optionnal_name", "api_key", "api_secret");
    let mut my_api = PoloniexApi::new(creds).unwrap();

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

```

For more examples, please see [examples](examples/).

## Testing
You can run the tests suite with `cargo test` for testing non private data
requests (this will ignore tests related to private requests).
You can use `cargo test --features "bitstamp_private_tests"` to run private
tests related to bitstamp exchange for example.
Before running private tests, make sure you have a `keys_real.json` file at the
root with the following structure :
```json
{
    "account_kraken": {
        "api_key"   : "123456789ABCDEF",
        "api_secret": "ABC&EF?abcdef"
    },
    "account_poloniex": {
        "api_key"   : "XYXY-XYXY-XYXY-XY",
        "api_secret": "A0A0B1B1C2C2"
    },
    "account_bitstamp": {
        "api_key"    : "XYXY-XYXY-XYXY-XY",
        "api_secret" : "A0A0B1B1C2C2",
        "customer_id": "123456"
    }
}
```
You must insert your real API keys, otherwise private tests may fail. No
action is performed if you run the tests : no test will open position, or
withdraw, etc.
Tests only check for correct authentication method and correct parsing.
You can examine the [tests](tests) folder just to be sure and look at the
[Cargo.toml](Cargo.toml) file for a complete list of features.


## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](LICENSE).

## Disclaimer
This SOFTWARE PRODUCT is provided by THE PROVIDER "as is" and "with all faults."
THE PROVIDER makes no representations or warranties of any kind concerning the
safety, suitability, lack of viruses, inaccuracies, typographical errors, or
other harmful components of this SOFTWARE PRODUCT. There are inherent dangers
in the use of any software, and you are solely responsible for determining
whether this SOFTWARE PRODUCT is compatible with your equipment and other
software installed on your equipment. You are also solely responsible for the
protection of your equipment and backup of your data, and THE PROVIDER will not
be liable for any damages you may suffer in connection with using, modifying,
or distributing this SOFTWARE PRODUCT.
