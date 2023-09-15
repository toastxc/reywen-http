use crate::{driver::Method, results::DeltaError, Delta};
#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde_json;

#[cfg(feature = "serde")]
impl Delta {
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        route: &str,
        data: Option<&str>,
    ) -> Result<T, DeltaError> {
        Delta::result(
            self.common(&format!("{}{}", self.url, route), method.into(), data)
                .await,
        )
            .await
    }

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
}
