use hyper::{
    body::HttpBody,
    header::{InvalidHeaderName, InvalidHeaderValue, CONTENT_TYPE, USER_AGENT},
    http::HeaderValue,
    Body, Method, Request, StatusCode,
};
use hyper_tls::HttpsConnector;

pub struct HyperBody {
    pub body: Option<Vec<u8>>,
    pub status: StatusCode,
}

pub enum Error {
    Engine(hyper::Error),
    Http(hyper::http::Error),
    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    StatusCode(StatusCode),
    HeaderName(InvalidHeaderName),
    HeaderValue(InvalidHeaderValue),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Default)]
pub struct Hyper {
    pub url: String,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub headers: hyper::HeaderMap,
}

impl HyperBody {
    #[cfg(feature = "serde")]
    pub fn serde_switch<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        match (self.body, self.status.as_u16()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(Error::StatusCode(self.status)),
        }
    }
}

impl Hyper {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn common(
        &self,
        method: Method,
        url: String,
        data: Option<Vec<u8>>,
    ) -> Result<HyperBody> {
        // http request
        let mut request = Request::builder().method(method).uri(url);
        let client = hyper::Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

        // headers
        let mut headers = self.headers.clone();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(self.user_agent.as_deref().unwrap_or(crate::USER_AGENT))?,
        );

        if let Some(content_type) = self.content_type.as_deref() {
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
        let response = client
            .request(request.body(data.map_or_else(Body::empty, Body::from))?)
            .await?;

        let status = response.status();

        let body = match response.into_body().data().await {
            None => None,
            Some(data) => Some(data?.to_vec()),
        };

        Ok(HyperBody { status, body })
    }

    pub async fn request_raw(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<HyperBody> {
        self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            data.into(),
        )
        .await
    }

    #[cfg(feature = "serde")]
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<T> {
        self.request_raw(method, path, data).await?.serde_switch()
    }
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Self::Engine(value)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(value: hyper::http::Error) -> Self {
        Self::Http(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(value: InvalidHeaderName) -> Self {
        Self::HeaderName(value)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::HeaderValue(value)
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
