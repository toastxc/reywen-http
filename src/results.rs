use crate::driver::Delta;
use hyper::StatusCode;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DeltaError {
    Http(hyper::StatusCode, String),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Byte(FromUtf8Error),
}

impl Delta {
    pub async fn result<T: serde::de::DeserializeOwned>(
        http: Result<hyper::Response<hyper::Body>, hyper::Error>,
    ) -> Result<T, DeltaError> {
        match http {
            Err(http) => Err(DeltaError::Hyper(http)),
            Ok(a) => {
                let (status, hyper_string) = Delta::hyper_data(a).await?;

                match status.as_u16() {
                    204 => Ok(serde_json::from_value(serde_json::Value::Null).unwrap()),
                    200 => match serde_json::from_str(&hyper_string) {
                        Ok(json) => Ok(json),
                        Err(a) => Err(DeltaError::Serde(a)),
                    },
                    _ => Err(DeltaError::Http(status, hyper_string)),
                }
            }
        }
    }
    pub async fn hyper_data(
        input: hyper::Response<hyper::Body>,
    ) -> Result<(StatusCode, String), DeltaError> {
        Ok((
            input.status(),
            match String::from_utf8(match hyper::body::to_bytes(input.into_body()).await {
                Ok(data) => data.to_vec(),
                Err(error) => return Err(DeltaError::Hyper(error)),
            }) {
                Ok(data) => data,
                Err(error) => return Err(DeltaError::Byte(error)),
            },
        ))
    }
}

// alias for result
pub async fn result<T: serde::de::DeserializeOwned>(
    http: Result<hyper::Response<hyper::Body>, hyper::Error>,
) -> Result<T, DeltaError> {
    Delta::result(http).await
}
