use std::string::FromUtf8Error;

use hyper::header::{InvalidHeaderName, InvalidHeaderValue};

use crate::results::{DeltaError, HeaderError};

pub trait ErrorConvert<T: std::fmt::Debug> {
    fn res(self) -> Result<T, DeltaError>;
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, hyper::Error> {
    fn res(self) -> Result<T, DeltaError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(DeltaError::Hyper(error)),
        }
    }
}
impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, InvalidHeaderName> {
    fn res(self) -> Result<T, DeltaError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(DeltaError::Header(HeaderError::Name(error))),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, InvalidHeaderValue> {
    fn res(self) -> Result<T, DeltaError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(DeltaError::Header(HeaderError::Value(error))),
        }
    }
}
impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, serde_json::Error> {
    fn res(self) -> Result<T, DeltaError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(DeltaError::Serde(error)),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, FromUtf8Error> {
    fn res(self) -> Result<T, DeltaError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(DeltaError::Byte(error)),
        }
    }
}

impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, hyper::http::Error> {
    fn res(self) -> Result<T, DeltaError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(DeltaError::HyperHTTP(error)),
        }
    }
}
