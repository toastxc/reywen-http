use crate::engines::DeltaRequest;
use crate::hyper_driver::{DeltaBody, DeltaResponse, Method};
use crate::{traits, Delta};

use async_trait::async_trait;
use hyper::{
    body::HttpBody,
    header::{CONTENT_TYPE, USER_AGENT},
    http::HeaderValue,
    Body, Request,
};
use hyper_tls::HttpsConnector;

#[cfg(feature = "serde")]
use crate::results::DeltaError;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Default)]
pub struct Hyper(Delta);

impl Hyper {
    pub fn new() -> Self {
        Self(Delta::new())
    }
}

#[async_trait]
impl DeltaRequest for Hyper {
    async fn common(&self, method: Method, url: String, data: Option<Vec<u8>>) -> DeltaResponse {
        // http request
        let mut request = Request::builder().method(method).uri(url);
        let client = hyper::Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

        // headers
        let mut headers = self.0.to_owned().headers;
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(
                self.0
                    .user_agent
                    .as_deref()
                    .unwrap_or("Reywen-HTTP/10.0 (async-tokio-runtime)"),
            )?,
        );
        if let Some(content_type) = self.0.content_type.as_deref() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type)?);
        };
        match request.headers_mut() {
            Some(original_headers) => original_headers.extend(headers),
            None => {
                for (key, value) in headers {
                    if let Some(key) = key {
                        request = request.header(key, value);
                    }
                }
            }
        }
        // request
        let a = client
            .request(request.body(match data {
                None => Body::empty(),
                Some(data) => Body::from(data),
            })?)
            .await?;

        let status = a.status();

        let body = match a.into_body().data().await {
            None => None,
            Some(data) => Some(data?.to_vec()),
        };

        Ok(DeltaBody {
            status: status.into(),
            body,
        })
    }

    async fn request_raw<'a>(
        &self,
        method: traits!(Method),
        path: traits!(String),
        data: traits!(Option<Vec<u8>>),
    ) -> DeltaResponse {
        self.common(method.into(), path.into(), data.into()).await
    }
    #[cfg(feature = "serde")]
    async fn request<'a, T: DeserializeOwned + Send + Sync>(
        &self,
        method: traits!(Method),
        path: traits!(String),
        data: traits!(Option<Vec<u8>>),
    ) -> Result<T, DeltaError> {
        self.request_raw(method, path, data).await?.serde_switcher()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn hyper_engine() {
        assert!(Hyper::new()
            .request_raw(Method::GET, "https://api.revolt.chat", None)
            .await
            .is_ok())
    }
}
