use serde_json::{Value, Map};
use bytes::Buf;
use bytes::buf::ext::Reader;
use crate::error::*;
use bytes::buf::BufExt as _;

pub fn deserialize_json(json_string: &str) -> Result<Map<String, Value>> {
    let data: Value = match serde_json::from_str(json_string) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    match data.as_object() {
        Some(value) => Ok(value.clone()),
        None => Err(ErrorKind::BadParse.into()),
    }
}

pub fn deserialize_json_r<B>(reader: Reader<B>) -> Result<Map<String, Value>> where B: Buf {
    let data: Value = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    match data.as_object() {
        Some(value) => Ok(value.clone()),
        None => Err(ErrorKind::BadParse.into()),
    }
}

pub fn deserialize_json_array(json_string: &str) -> Result<Map<String, Value>> {
    let data: Value = match serde_json::from_str(json_string) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    if data.is_array() {
        let mut map = Map::new();
        map.insert("data".to_string(), data);
        Ok(map)
    }

    else {
        Err(ErrorKind::BadParse.into())
    }
}

/// Convert a JSON array into a map containing a Vec for the "data" key
pub fn deserialize_json_array_r<B>(reader: Reader<B>) -> Result<Map<String, Value>> where B: Buf {
    let data: Value = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    if data.is_array() {
        let mut map = Map::new();
        map.insert("data".to_string(), data);
        Ok(map)
    }

    else {
        Err(ErrorKind::BadParse.into())
    }
}
