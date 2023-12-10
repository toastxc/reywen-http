use crate::engines::reqwest::{
    results::{Error, Result},
    Reqwest, ReqwestBody,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Body, Client, Method, Request, Url,
};
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;
use std::str::FromStr;
#[cfg(feature = "serde")]
// impl ReqwestBody {
//     fn serde_switch<T: DeserializeOwned>(self) -> Result<T> {
//         match (self.body, self.status.as_u16()) {
//             (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
//             (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
//             _ => Err(Error::StatusCode(self.status)),
//         }
//     }
// }
impl ReqwestBody {
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

impl Reqwest {
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
        self.common(method.into(), path.into(), data.into())
            .await?
            .serde_switch()
    }

    pub async fn common(
        &self,
        method: Method,
        path: String,
        data: Option<Vec<u8>>,
    ) -> Result<ReqwestBody> {
        let mut request = Request::new(method, Url::from_str(&format!("{}{path}", self.url))?);
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

        let body = response.bytes().await?.to_vec().into();
        Ok(ReqwestBody { body, status })
    }

    pub async fn request_raw(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<Vec<u8>> {
        Ok(self
            .common(
                method.into(),
                format!("{}{}", self.url, path.into()),
                data.into(),
            )
            .await?
            .body
            .get_or_insert_with(Vec::new)
            .to_vec())
    }
}
