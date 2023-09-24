use crate::hyper_driver::{Delta, HyperResponse, Method};
#[cfg(feature = "serde")]
use crate::serde::ResponseSerde;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;

impl Delta {
    pub async fn request_raw(
        &self,
        method: Method,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> HyperResponse {
        self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            data.into(),
        )
        .await
    }
}

#[cfg(feature = "serde")]
impl Delta {
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> ResponseSerde<T> {
        Self::result_convert(self.common(method.into(), path.into(), data.into()).await).await
    }
}
