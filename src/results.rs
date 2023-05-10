use reqwest::Response;

use crate::driver::Delta;

#[derive(Debug)]
pub enum DeltaError {
    HTTP(reqwest::StatusCode, String),
    REQWEST(reqwest::Error),
    SERDE(reqwest::Error),
}

impl Delta {
    pub async fn result<T: serde::de::DeserializeOwned>(
        http: Result<Response, reqwest::Error>,
    ) -> Result<T, DeltaError> {
        let res = http;
        let result: T = match res {
            Err(http) => {
                return Err(DeltaError::REQWEST(http));
            }
            Ok(a) => {
                if !a.status().is_success() {
                    return Err(DeltaError::HTTP(
                        a.status(),
                        a.text().await.unwrap_or_default(),
                    ));
                }
                if a.status() == 204 {
                    return Ok(serde_json::from_value(serde_json::Value::Null).unwrap());
                }

                match a.json().await {
                    Ok(a) => a,
                    Err(a) => return Err(DeltaError::SERDE(a)),
                }
            }
        };
        Ok(result)
    }
}
#[derive(Debug)]
pub enum HeaderError {
    Name(reqwest::header::InvalidHeaderName),
    Value(reqwest::header::InvalidHeaderValue),
    Generic(reqwest::Error),
}
