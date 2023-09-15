use crate::driver::{Delta2, Response};
use crate::results::DeltaError;
use hyper::body::{Bytes, HttpBody};
use hyper::Error;
use serde::de::DeserializeOwned;

pub type ResponseSerde<T> = Result<T, DeltaError>;

impl Delta2 {
    pub async fn result_convert<T: DeserializeOwned>(input: Response) -> ResponseSerde<T> {
        let input = input?;
        let (status_int, status) = (input.status().as_u16(), input.status());

        let data = match input.into_body().data().await {
            None => return Ok(serde_json::from_value(serde_json::Value::Null).unwrap()),
            Some(Ok(data)) => data.to_vec(),
            Some(Err(error)) => return Err(error.into()),
        };

        match (status_int, status) {
            (200, _) => Ok(serde_json::from_slice(&data)?),
            (204, _) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            (_, error) => Err(DeltaError::Http(
                error,
                String::from_utf8_lossy(&data).to_string(),
            )),
        }
    }
}
