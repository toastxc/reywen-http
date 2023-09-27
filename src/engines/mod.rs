#[cfg(feature = "serde")]
use crate::results::DeltaError;

#[cfg(feature = "serde")]
use crate::DeltaBody;

use crate::DeltaResponse;
use async_trait::async_trait;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;

#[cfg(feature = "hyper_engine")]
pub mod hyper;

pub enum Method {
    POST,
    PUT,
    PATCH,
    GET,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

#[macro_export]
macro_rules! traits {
    ($t:ty) => {
        impl Into<$t> + Sync  + Send + 'a
    };
}

#[async_trait]
pub trait DeltaRequest<HttpE, HN, HV> {
    #[cfg(feature = "serde")]
    async fn request<'a, T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        method: traits!(Method),
        path: traits!(String),
        data: traits!(Option<Vec<u8>>),
    ) -> Result<T, DeltaError<HttpE, HN, HV>> {
        self.request_raw(method, path, data).await?.serde_switcher()
    }

    async fn request_raw<'a>(
        &self,
        method: traits!(Method),
        path: traits!(String),
        data: traits!(Option<Vec<u8>>),
    ) -> DeltaResponse<HttpE, HN, HV>;

    async fn common(
        &self,
        method: Method,
        url: String,
        data: Option<Vec<u8>>,
    ) -> DeltaResponse<HttpE, HN, HV>;
}

#[cfg(feature = "serde")]
impl DeltaBody {
    /// Generates the JSON payload of the body deserialized using `serde`.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if the
    /// body is not valid JSON or the status code is not 200 or 204.
    pub fn serde_switcher<T: DeserializeOwned, HttpE, HN, HV>(
        self,
    ) -> Result<T, DeltaError<HttpE, HN, HV>> {
        match (self.body, self.status.as_u16()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(DeltaError::StatusCode(self.status)),
        }
    }
}
