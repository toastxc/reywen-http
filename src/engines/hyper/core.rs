use crate::{
    engines::hyper::Error,
    engines::hyper::Result,
    engines::hyper::{Body, Hyper},
};
use hyper::{
    header::HeaderValue,
    header::{CONTENT_TYPE, USER_AGENT},
    http::HeaderName,
    HeaderMap, Method, Request,
};
use hyper_tls::HttpsConnector;
use std::str::FromStr;

impl Hyper {
    pub fn set_url(&mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self.to_owned()
    }
    pub fn set_user_agent(&mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self.to_owned()
    }
    pub fn set_content_type(&mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self.to_owned()
    }
    pub fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<Self> {
        self.headers.append(
            HeaderName::from_str(key.into().as_str())?,
            HeaderValue::from_str(value.into().as_str())?,
        );
        Ok(self.to_owned())
    }
    pub fn set_headers(&mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self.to_owned()
    }
}

// methods
impl Body {
    #[cfg(feature = "serde")]
    pub fn serde_switch<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        match (self.body, self.status.as_u16()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(Error::StatusCode(self.status)),
        }
    }
    pub fn bytes(self) -> Result<Vec<u8>> {
        match (self.body, self.status.as_u16()) {
            (Some(data), 200) => Ok(data),
            (None, 204) | (None, 200) => Ok(Vec::new()),
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
        path: String,
        data: Option<Vec<u8>>,
    ) -> Result<Body> {
        // http request
        let uri = format!("{}{path}", self.url);
        let mut request = Request::builder().method(method).uri(uri);

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
            .request(request.body(data.map_or_else(hyper::Body::empty, hyper::Body::from))?)
            .await?;

        Ok(Body {
            status: response.status(),
            body: hyper::body::to_bytes(response.into_body())
                .await?
                .to_vec()
                .into(),
        })
    }

    pub async fn request_raw(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<Vec<u8>> {
        self.common(method.into(), path.into(), data.into())
            .await?
            .bytes()
    }

    #[cfg(feature = "serde")]
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<T> {
        self.common(method.into(), path.into(), data.into())
            .await?
            .serde_switch()
    }
}
