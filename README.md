![Coinnect](/coinnect.png)
==========================

Coinnect is a Rust library aiming to provide a complete access to REST APIs for various crypto-currencies exchanges.
Currently only Poloniex is supported but other exchanges will be added soon.
All methods consume HTTPS api. This library is not intendeed to stream data
(you should use websocket/FIX in that case).

The project is dual licensed under the terms of the Apache License, Version 2.0,
and the MIT License.

**WARNING:**  This library is highly experimental at the moment. Please do not invest what you can't afford to loose.

### Exchange support:
- [x] Poloniex (partial)
- [x] Kraken (partial)
- [ ] Bitstamp
- [ ] Whaleclub
- [ ] ...

Feel free to make a PR to add support to your favorite exchange ;)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
coinnect = "0.1"
```

and this to your crate root:

```rust
extern crate coinnect;
```

## Example

The example below show you how to connect to Poloniex

```rust
extern crate coinnect;

use coinnect::poloniex::PoloniexApi;

fn main() {
    // We create a PoloniexApi by providing API key/secret
    // You can give an empty String if you only use public methods
    let mut my_api = PoloniexApi::new("api_key".to_string(), "api_secret".to_string());

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

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

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
