//! Contains the Poloniex credentials.

use std::collections::HashMap;
use std::str::FromStr;

use serde_json;
use serde_json::Value;

use coinnect::Credentials;
use exchange::Exchange;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use error::*;

#[derive(Debug)]
pub struct PoloniexCreds {
    exchange: Exchange,
    name: String,
    data: HashMap<String, String>,
}

impl PoloniexCreds {
    /// Create a new `PoloniexCreds` from arguments.
    pub fn new(name: &str, api_key: &str, api_secret: &str) -> Self {
        let mut creds = PoloniexCreds {
            data: HashMap::new(),
            exchange: Exchange::Poloniex,
            name: if name.is_empty() {
                "PoloniexClient".to_string()
            } else {
                name.to_string()
            },
        };


        //if api_key.is_empty() {
        //warning!("No API key set for the Bistamp client");
        //}
        creds
            .data
            .insert("api_key".to_string(), api_key.to_string());

        //if api_secret.is_empty() {
        //warning!("No API secret set for the Bistamp client");
        //}
        creds
            .data
            .insert("api_secret".to_string(), api_secret.to_string());

        creds
    }


    /// Create a new `PoloniexCreds` from a json configuration file. This file must follow this
    /// structure:
    ///
    /// ```json
    /// {
    ///     "account_kraken": {
    ///         "exchange"  : "kraken",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef"
    ///     },
    ///     "account_bitstamp": {
    ///         "exchange"   : "bitstamp",
    ///         "api_key"    : "1234567890ABCDEF1234567890ABCDEF",
    ///         "api_secret" : "1234567890ABCDEF1234567890ABCDEF",
    ///         "customer_id": "123456"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Poloniex account with
    /// `PoloniexAPI::new(PoloniexCreds::new_from_file("account_kraken", Path::new("/keys.json")))`
    pub fn new_from_file(name: &str, path: PathBuf) -> Result<Self> {
        let mut f = File::open(&path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let data: Value = serde_json::from_str(&buffer)?;
        let json_obj = data.as_object()
            .ok_or_else(|| ErrorKind::BadParse)?
            .get(name)
            .ok_or_else(|| ErrorKind::MissingField(name.to_string()))?;
        let api_key = json_obj
            .get("api_key")
            .ok_or_else(|| ErrorKind::MissingField("api_key".to_string()))?
            .as_str()
            .ok_or_else(|| ErrorKind::InvalidFieldFormat("api_key".to_string()))?;
        let api_secret =
            json_obj
                .get("api_secret")
                .ok_or_else(|| ErrorKind::MissingField("api_secret".to_string()))?
                .as_str()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat("api_secret".to_string()))?;
        let exchange = {
            let exchange_str =
                json_obj
                    .get("exchange")
                    .ok_or_else(|| ErrorKind::MissingField("customer_id".to_string()))?
                    .as_str()
                    .ok_or_else(|| ErrorKind::InvalidFieldFormat("customer_id".to_string()))?;

            Exchange::from_str(exchange_str)
                .chain_err(|| ErrorKind::InvalidFieldValue("exchange".to_string()))?
        };

        if exchange != Exchange::Poloniex {
            return Err(ErrorKind::InvalidConfigType(Exchange::Poloniex, exchange).into());
        }

        Ok(PoloniexCreds::new(name, api_key, api_secret))
    }
}

impl Credentials for PoloniexCreds {
    /// Return a value from the credentials.
    fn get(&self, key: &str) -> Option<String> {
        if let Some(res) = self.data.get(key) {
            Some(res.clone())
        } else {
            None
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn exchange(&self) -> Exchange {
        self.exchange
    }
}
