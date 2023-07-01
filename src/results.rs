use crate::{traits::ErrorConvert, Delta};
use hyper::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    StatusCode,
};
use std::string::FromUtf8Error;

#[cfg(feature = "serde")]
#[derive(Debug)]
pub enum DeltaError {
    Http(hyper::StatusCode, String),
    Hyper(hyper::Error),
    HyperHTTP(hyper::http::Error),
    Serde(serde_json::Error),
    Byte(FromUtf8Error),
    Header(HeaderError),
}

#[cfg(not(feature = "serde"))]
#[derive(Debug)]
pub enum DeltaError {
    Http(hyper::StatusCode, String),
    Hyper(hyper::Error),
    HyperHTTP(hyper::http::Error),
    Byte(FromUtf8Error),
    Header(HeaderError),
}

#[derive(Debug)]
pub enum HeaderError {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

impl Delta {
    pub async fn result_raw(
        http: Result<hyper::Response<hyper::Body>, DeltaError>,
    ) -> Result<Vec<u8>, DeltaError> {
        let (status, hyper_string) = Delta::hyper_data_raw(http?).await?;

        match status.as_u16() {
            204 => Ok(Vec::new()),
            200 => Ok(hyper_string),
            _ => Err(DeltaError::Http(status, String::new())),
        }
    }

    pub async fn hyper_data_raw(
        input: hyper::Response<hyper::Body>,
    ) -> Result<(StatusCode, Vec<u8>), DeltaError> {
        Ok((
            input.status(),
            hyper::body::to_bytes(input.into_body())
                .await
                .res()?
                .to_vec(),
        ))
    }

    pub async fn hyper_data(
        input: hyper::Response<hyper::Body>,
    ) -> Result<(StatusCode, String), DeltaError> {
        let (status, vec) = Self::hyper_data_raw(input).await?;

        Ok((status, String::from_utf8(vec).res()?))
    }
}
