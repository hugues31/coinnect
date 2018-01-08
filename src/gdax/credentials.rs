//! Contains the Gdax credentials.

use serde_json;
use serde_json::Value;

use coinnect::Credentials;
use exchange::Exchange;
use helpers;
use error::*;

use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GdaxCreds {
    exchange: Exchange,
    name: String,
    data: HashMap<String, String>,
}

impl GdaxCreds {
    /// Create a new `GdaxCreds` from a json configuration file. This file must follow this
    /// structure:
    ///
    /// ```json
    /// {
    ///     "account_gdax": {
    ///         "exchange"  : "gdax",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef",
    ///         "passphrase": "123456"
    ///     },
    ///     "account_bitstamp": {
    ///         "exchange"   : "bitstamp",
    ///         "api_key"    : "1234567890ABCDEF1234567890ABCDEF",
    ///         "api_secret" : "1234567890ABCDEF1234567890ABCDEF",
    ///         "customer_id": "123456"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Gdax account with
    /// `GdaxAPI::new(GdaxCreds::new_from_file("account_gdax", Path::new("/keys.json")))`
    pub fn new_from_file(name: &str, path: PathBuf) -> Result<Self> {
        let mut f = File::open(&path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let data: Value = serde_json::from_str(&buffer)?;
        let json_obj = data.as_object()
            .ok_or_else(|| ErrorKind::BadParse)?
            .get(name)
            .ok_or_else(|| ErrorKind::MissingField(name.to_string()))?;

        let api_key = helpers::get_json_string(json_obj, "api_key")?;
        let api_secret = helpers::get_json_string(json_obj, "api_secret")?;
        let passphrase = helpers::get_json_string(json_obj, "passphrase")?;
        let exchange = {
            let exchange_str = helpers::get_json_string(json_obj, "exchange")?;
            Exchange::from_str(exchange_str)
                .chain_err(|| ErrorKind::InvalidFieldValue("exchange".to_string()))?
        };

        if exchange != Exchange::Gdax {
            return Err(ErrorKind::InvalidConfigType(Exchange::Gdax, exchange).into());
        }

        Ok(GdaxCreds::new(name, api_key, api_secret, passphrase))
    }


    /// Create a new `GdaxCreds` from arguments.
    pub fn new(name: &str, api_key: &str, api_secret: &str, passphrase: &str) -> Self {
        let mut creds = GdaxCreds {
            data: HashMap::new(),
            exchange: Exchange::Gdax,
            name: if name.is_empty() {
                "GdaxClient".to_string()
            } else {
                name.to_string()
            },
        };


        //if api_key.is_empty() {
        //warning!("No API key set for the Gdax client");
        //}
        creds
            .data
            .insert("api_key".to_string(), api_key.to_string());

        //if api_secret.is_empty() {
        //warning!("No API secret set for the Gdax client");
        //}
        creds
            .data
            .insert("api_secret".to_string(), api_secret.to_string());

        //if api_secret.is_empty() {
        //warning!("No API customer ID set for the Gdax client");
        //}
        creds
            .data
            .insert("passphrase".to_string(), passphrase.to_string());

        creds
    }
}

impl Credentials for GdaxCreds {
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
