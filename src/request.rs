#[cfg(feature = "serde")]
use crate::serde::ResponseSerde;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;
use std::fmt::format;

use crate::driver::{Delta2, Method, Response};
use crate::results::NON_STR;

impl Delta2 {
    pub async fn request_raw(
        &self,
        method: Method,
        path: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) -> Response {
        self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            Some(data),
        )
        .await
    }
    pub async fn request_raw_empty(&self, method: Method, path: impl Into<String>) -> Response {
        self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            NON_STR,
        )
        .await
    }
}

#[cfg(feature = "serde")]
impl Delta2 {
    pub async fn request_empty<T: DeserializeOwned>(
        &self,
        method: Method,
        path: impl Into<String>,
    ) -> ResponseSerde<T> {
        Self::result_convert(
            self.common(
                method.into(),
                format!("{}{}", self.url, path.into()),
                NON_STR,
            )
            .await,
        )
        .await
    }
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: impl Into<String>,
        body: impl Into<Vec<u8>>,
    ) -> ResponseSerde<T> {
        Self::result_convert(
            self.common(
                method.into(),
                format!("{}{}", self.url, path.into()),
                Some(body),
            )
            .await,
        )
        .await
    }
}
