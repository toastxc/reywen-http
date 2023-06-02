use crate::{driver::Delta, traits::ErrorConvert};
use hyper::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    StatusCode,
};
use std::string::FromUtf8Error;
#[derive(Debug)]
pub enum DeltaError {
    Http(hyper::StatusCode, String),
    Hyper(hyper::Error),
    HyperHTTP(hyper::http::Error),
    Serde(serde_json::Error),
    Byte(FromUtf8Error),
    Header(HeaderError),
}
#[derive(Debug)]
pub enum HeaderError {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

impl Delta {
    pub async fn result<T: serde::de::DeserializeOwned>(
        http: Result<hyper::Response<hyper::Body>, DeltaError>,
    ) -> Result<T, DeltaError> {
        let (status, hyper_string) = Delta::hyper_data(http?).await?;

        match status.as_u16() {
            204 => Ok(serde_json::from_value(serde_json::Value::Null).unwrap()),
            200 => match serde_json::from_str(&hyper_string) {
                Ok(json) => Ok(json),
                Err(a) => Err(DeltaError::Serde(a)),
            },
            _ => Err(DeltaError::Http(status, hyper_string)),
        }
    }
    pub async fn hyper_data(
        input: hyper::Response<hyper::Body>,
    ) -> Result<(StatusCode, String), DeltaError> {
        Ok((
            input.status(),
            String::from_utf8(
                hyper::body::to_bytes(input.into_body())
                    .await
                    .res()?
                    .to_vec(),
            )
            .res()?,
        ))
    }
}

// alias for result
pub async fn result<T: serde::de::DeserializeOwned>(
    http: Result<hyper::Response<hyper::Body>, DeltaError>,
) -> Result<T, DeltaError> {
    Delta::result(http).await
}
