use crate::hyper_driver::{DeltaBody, DeltaResponse, Method};
#[cfg(feature = "serde")]
use crate::results::DeltaError;
use async_trait::async_trait;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;
pub mod hyper;

#[macro_export]
macro_rules! traits {
    ($t:ty) => {
        impl Into<$t> + Sync  + Send + 'a
    };
}

#[async_trait]
pub trait DeltaRequest {
    #[cfg(feature = "serde")]
    async fn request<'a, T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        method: traits!(Method),
        path: traits!(String),
        data: traits!(Option<Vec<u8>>),
    ) -> Result<T, DeltaError>;

    async fn request_raw<'a>(
        &self,
        method: traits!(Method),
        path: traits!(String),
        data: traits!(Option<Vec<u8>>),
    ) -> DeltaResponse;

    async fn common(&self, method: Method, url: String, data: Option<Vec<u8>>) -> DeltaResponse;
}

impl DeltaBody {
    #[cfg(feature = "serde")]
    pub fn serde_switcher<T: DeserializeOwned>(self) -> Result<T, DeltaError> {
        match (self.body, self.status.as_u16()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(DeltaError::StatusCode(self.status)),
        }
    }
}
