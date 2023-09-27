use std::string::FromUtf8Error;

use crate::StatusCode;

#[derive(Debug)]
pub enum DeltaError<HttpE, HN, HV> {
    StatusCode(StatusCode),
    HTTP(HttpE),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    Byte(FromUtf8Error),
    Header(HeaderError<HN, HV>),
    Engine(Engine),
}

#[derive(Debug)]
pub enum Engine {
    #[cfg(feature = "hyper_engine")]
    Hyper(hyper::Error),
}

#[derive(Debug)]
pub enum HeaderError<N, V> {
    Name(N),
    Value(V),
}

#[cfg(feature = "serde")]
impl<HttpE, HN, HV> From<serde_json::Error> for DeltaError<HttpE, HN, HV> {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
