use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    Engine(reqwest::Error),
    Url(url::ParseError),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    HeaderValue(InvalidHeaderValue),
    HeaderName(InvalidHeaderName),
    StatusCode(StatusCode),
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Engine(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::HeaderValue(value)
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(value: InvalidHeaderName) -> Self {
        Self::HeaderName(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
