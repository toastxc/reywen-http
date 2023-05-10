use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::results::HeaderError;

pub struct HeaderData {
    str: (String, String),
    header_tuple: (HeaderName, HeaderValue),
    header_map: HeaderMap,
}

impl HeaderData {
    pub fn new(key: &str, value: &str) -> Result<Self, HeaderError> {
        let str = (key.to_string(), value.to_string());
        let headername = match reqwest::header::HeaderName::from_bytes(str.0.as_bytes().clone()) {
            Ok(valid_key) => valid_key,
            Err(error) => return Err(HeaderError::Name(error)),
        };
        let headervalue = match reqwest::header::HeaderValue::from_str(value) {
            Ok(valid_value) => valid_value,
            Err(error) => return Err(HeaderError::Value(error)),
        };
        let mut header_map: HeaderMap = HeaderMap::new();
        let header_tuple = (headername, headervalue);
        header_map.insert(header_tuple.0.clone(), header_tuple.1.clone());

        Ok(Self {
            str,
            header_tuple,
            header_map,
        })
    }

    pub fn as_str(&self) -> (String, String) {
        self.str.to_owned()
    }
    pub fn as_header_value(&self) -> (HeaderName, HeaderValue) {
        self.header_tuple.to_owned()
    }
    pub fn as_header_map(&self) -> HeaderMap {
        self.header_map.to_owned()
    }
}
