#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::engines::Setter;
use reqwest::{
    header::{HeaderValue, InvalidHeaderName, InvalidHeaderValue, CONTENT_TYPE, USER_AGENT},
    Body, Client, Method, Request, StatusCode, Url,
};

#[derive(Debug, Clone, Default)]
pub struct Reqwest {
    pub url: String,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub headers: reqwest::header::HeaderMap,
}

#[derive(Debug)]
pub enum Error {
    Engine(reqwest::Error),
    Url(url::ParseError),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    HeaderValue(InvalidHeaderValue),
    HeaderName(InvalidHeaderName),
    StatusCode(StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Engine(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::HeaderValue(value)
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(value: InvalidHeaderName) -> Self {
        Self::HeaderName(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

#[derive(Debug, Clone)]
pub struct ReqwestBody {
    pub body: Option<Vec<u8>>,
    pub status: StatusCode,
}

#[cfg(feature = "serde")]
impl ReqwestBody {
    fn serde_switch<T: DeserializeOwned>(self) -> Result<T> {
        match (self.body, self.status.as_u16()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(Error::StatusCode(self.status)),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Reqwest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "serde")]
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<T> {
        self.request_raw(method, path, data).await?.serde_switch()
    }

    pub async fn common(
        &self,
        method: Method,
        url: String,
        data: Option<Vec<u8>>,
    ) -> Result<ReqwestBody> {
        let mut request = Request::new(method, Url::from_str(&url)?);
        let client = Client::new();

        let headers = request.headers_mut();

        headers.extend(self.headers.clone());
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(self.user_agent.as_deref().unwrap_or(crate::USER_AGENT))?,
        );

        if let Some(content_type) = self.content_type.as_deref() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type)?);
        }
        *request.body_mut() = data.map(Body::from);

        let response = client.execute(request).await?;
        let status = response.status();

        let body = response.bytes().await?.to_vec();
        let body = if body.is_empty() { None } else { Some(body) };

        Ok(ReqwestBody { body, status })
    }

    pub async fn request_raw(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<ReqwestBody> {
        self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            data.into(),
        )
        .await
    }
}

impl Setter for Reqwest {
    fn set_url(&mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self.clone()
    }
    fn set_user_agent(&mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self.clone()
    }
    fn set_content_type(&mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::Reqwest;
    use reqwest::Method;

    #[tokio::test]
    async fn request_basic() {
        assert!(Reqwest::new()
            .request_raw(Method::GET, "https://api.revolt.chat", None)
            .await
            .is_ok());
    }
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn request_basic() {
    assert!(Reqwest::new()
        .request_raw(Method::GET, "https://api.revolt.chat", None)
        .await
        .is_ok());
}
