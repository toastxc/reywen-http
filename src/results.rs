use crate::driver::{Delta2, Method, Response};
use hyper::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    StatusCode,
};
use std::string::FromUtf8Error;


pub const NON_STR: Option<&str> = None;
#[derive(Debug)]
pub enum DeltaError {
    Http(StatusCode, String),
    Hyper(hyper::Error),
    HyperHTTP(hyper::http::Error),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    Byte(FromUtf8Error),
    Header(HeaderError),
}

#[derive(Debug)]
pub enum HeaderError {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

