use crate::results::{DeltaError, HeaderError};
use hyper::header::InvalidHeaderValue;
use hyper::http::Error;

impl From<InvalidHeaderValue> for DeltaError {
    fn from(value: InvalidHeaderValue) -> Self {
        DeltaError::Header(HeaderError::Value(value))
    }
}


impl From<hyper::http::Error> for  DeltaError {
    fn from(value: Error) -> Self {
        DeltaError::HyperHTTP(value)
    }
}


impl From<hyper::Error> for DeltaError {
    fn from(value: hyper::Error) -> Self {
        Self::Hyper(value)
    }
}