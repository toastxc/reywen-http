use crate::hyper_driver;
use crate::hyper_driver::StatusCode;
use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use std::num::NonZeroU16;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DeltaError {
    StatusCode(StatusCode),
    HTTP(hyper::http::Error),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    Byte(FromUtf8Error),
    Header(HeaderError),
    Engine(Engine),
}

#[derive(Debug)]
pub enum Engine {
    Hyper(hyper::Error),
}

#[derive(Debug)]
pub enum HeaderError {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

impl From<InvalidHeaderValue> for DeltaError {
    fn from(value: InvalidHeaderValue) -> Self {
        DeltaError::Header(HeaderError::Value(value))
    }
}
#[cfg(feature = "serde")]
impl From<serde_json::Error> for DeltaError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
impl From<hyper::http::Error> for DeltaError {
    fn from(value: hyper::http::Error) -> Self {
        DeltaError::HTTP(value)
    }
}
impl From<hyper::Error> for DeltaError {
    fn from(value: hyper::Error) -> Self {
        Self::Engine(Engine::Hyper(value))
    }
}

impl From<hyper::StatusCode> for hyper_driver::StatusCode {
    fn from(value: hyper::StatusCode) -> Self {
        StatusCode(NonZeroU16::new(value.as_u16()).unwrap())
    }
}
