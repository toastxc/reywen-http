use std::num::NonZeroU16;

use super::Method;
use crate::results::{Engine, HeaderError};
use crate::{engines::DeltaRequest, Delta};
use crate::{traits, DeltaBody, DeltaResponse, StatusCode};

use async_trait::async_trait;
use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use hyper::{
    body::HttpBody,
    header::{CONTENT_TYPE, USER_AGENT},
    http::HeaderValue,
    Body, Request,
};
use hyper_tls::HttpsConnector;

#[cfg(feature = "serde")]
use crate::results::DeltaError;

pub type Response<HttpE, HN, HV> = Result<hyper::Response<hyper::Body>, DeltaError<HttpE, HN, HV>>;

#[derive(Debug, Clone, Default)]
pub struct Hyper(Delta);

impl Hyper {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Method> for hyper::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::POST => Self::POST,
            Method::PUT => Self::PUT,
            Method::PATCH => Self::PATCH,
            Method::GET => Self::GET,
            Method::DELETE => Self::DELETE,
            Method::HEAD => Self::HEAD,
            Method::OPTIONS => Self::OPTIONS,
            Method::CONNECT => Self::CONNECT,
            Method::TRACE => Self::TRACE,
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<hyper::StatusCode> for StatusCode {
    fn from(value: hyper::StatusCode) -> Self {
        // `new_unchecked` is used because `hyper::StatusCode::as_u16` is guaranteed
        // to return a non-zero unsigned 16-bit value.
        Self(unsafe { NonZeroU16::new_unchecked(value.as_u16())})
    }
}

#[async_trait]
impl DeltaRequest<hyper::http::Error, InvalidHeaderName, InvalidHeaderValue> for Hyper {
    async fn common(
        &self,
        method: Method,
        url: String,
        data: Option<Vec<u8>>,
    ) -> DeltaResponse<hyper::http::Error, InvalidHeaderName, InvalidHeaderValue> {
        // http request
        let mut request = Request::builder().method(method).uri(url);
        let client = hyper::Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

        // headers
        let mut headers = self.0.clone().headers;
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(
                self.0
                    .user_agent
                    .as_deref()
                    .unwrap_or("Reywen-HTTP/10.0 (async-tokio-runtime)"),
            )
            .map_err(|error| DeltaError::Header(HeaderError::Value(error)))?,
        );

        if let Some(content_type) = self.0.content_type.as_deref() {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(content_type)
                    .map_err(|error| DeltaError::Header(HeaderError::Value(error)))?,
            );
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
        let response = client
            .request(
                match request.body(data.map_or_else(Body::empty, Body::from)) {
                    Ok(request) => request,
                    Err(error) => return Err(DeltaError::HTTP(error)),
                },
            )
            .await?;

        let status = response.status();

        let body = match response.into_body().data().await {
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
    ) -> DeltaResponse<hyper::http::Error, InvalidHeaderName, InvalidHeaderValue> {
        self.common(
            method.into(),
            format!("{}{}", self.0.url, path.into()),
            data.into(),
        )
        .await
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
            .is_ok());
    }
}

impl<HN, HV> From<hyper::http::Error> for DeltaError<hyper::http::Error, HN, HV> {
    fn from(value: hyper::http::Error) -> Self {
        Self::HTTP(value)
    }
}

impl<HttpE, HN, HV> From<hyper::Error> for DeltaError<HttpE, HN, HV> {
    fn from(value: hyper::Error) -> Self {
        Self::Engine(Engine::Hyper(value))
    }
}
