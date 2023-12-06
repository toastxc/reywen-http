use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use hyper::StatusCode;

#[derive(Debug)]
pub enum Error {
    Engine(hyper::Error),
    Http(hyper::http::Error),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    StatusCode(StatusCode),
    HeaderName(InvalidHeaderName),
    HeaderValue(InvalidHeaderValue),
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Self::Engine(value)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(value: hyper::http::Error) -> Self {
        Self::Http(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(value: InvalidHeaderName) -> Self {
        Self::HeaderName(value)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::HeaderValue(value)
    }
}
