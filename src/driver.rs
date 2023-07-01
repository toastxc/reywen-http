use super::traits::ErrorConvert;
use crate::{results::DeltaError, Delta};
use hyper::{
    header::{self, USER_AGENT},
    http::{HeaderName, HeaderValue},
    Request,
};

use hyper_tls::HttpsConnector;

use hyper::Client;

impl Delta {
    pub fn set_user_agent(&mut self, user_agent: &str) -> Self {
        self.user_agent = Some(String::from(user_agent));
        self.to_owned()
    }
    pub fn set_url(&mut self, url: &str) -> Self {
        self.url = String::from(url);
        self.to_owned()
    }

    pub fn set_timeout(&mut self, timout: u64) -> Self {
        self.timeout = std::time::Duration::from_secs(timout);
        self.to_owned()
    }

    pub fn set_headers(&mut self, headers: Vec<(&str, &str)>) -> Result<Self, DeltaError> {
        self.remove_headers();
        self.add_headers(headers)?;
        Ok(self.to_owned())
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> Result<Self, DeltaError> {
        let (key, value) = Self::kv_parse(key, value)?;
        self.headers.insert(key, value);

        Ok(self.to_owned())
    }

    pub fn add_headers(&mut self, headers: Vec<(&str, &str)>) -> Result<Self, DeltaError> {
        for (key, value) in headers.clone() {
            let (key, value) = Self::kv_parse(key, value)?;
            self.headers.append(key, value);
        }
        Ok(self.to_owned())
    }
    pub fn remove_headers(&mut self) -> Self {
        self.headers = header::HeaderMap::new();
        self.to_owned()
    }

    pub async fn get(&self, route: &str) -> Response {
        self.common(&format!("{}{}", self.url, route), hyper::Method::GET, None)
            .await
    }

    pub async fn post(&self, route: &str, data: Option<&str>) -> Response {
        self.common(&format!("{}{}", self.url, route), hyper::Method::POST, data)
            .await
    }

    pub async fn put(&self, route: &str, data: Option<&str>) -> Response {
        self.common(&format!("{}{}", self.url, route), hyper::Method::PUT, data)
            .await
    }

    pub async fn delete(&self, route: &str, data: Option<&str>) -> Response {
        self.common(
            &format!("{}{}", self.url, route),
            hyper::Method::DELETE,
            data,
        )
        .await
    }
    pub async fn patch(&self, route: &str, data: Option<&str>) -> Response {
        self.common(
            &format!("{}{}", self.url, route),
            hyper::Method::PATCH,
            data,
        )
        .await
    }

    fn kv_parse(key: &str, value: &str) -> Result<(HeaderName, HeaderValue), DeltaError> {
        let a = hyper::header::HeaderName::from_bytes(key.as_bytes()).res()?;
        let b = hyper::header::HeaderValue::from_str(value).res()?;

        Ok((a, b))
    }

    async fn common(&self, url: &str, method: hyper::Method, input_data: Option<&str>) -> Response {
        let https = HttpsConnector::new();

        let client = Client::builder().build::<_, hyper::Body>(https);

        let mut headers = hyper::HeaderMap::new();

        let user_agent = (
            USER_AGENT,
            &self
                .clone()
                .user_agent
                .unwrap_or(String::from("Reywen-HTTP/10.0 (async-tokio-runtime)")),
        );

        headers.insert(user_agent.0, HeaderValue::from_str(user_agent.1).res()?);

        let (body, content_type) = match input_data {
            Some(data) => (
                hyper::body::Body::from(data.to_owned()),
                self.content_type
                    .clone()
                    .unwrap_or(String::from("application/json")),
            ),
            None => (
                hyper::Body::empty(),
                self.content_type.clone().unwrap_or(String::new()),
            ),
        };

        for (key, value) in self.headers.clone() {
            if let Some(key) = key {
                headers.insert(key, value);
            };
        }

        let mut request = Request::builder().method(method).uri(url);
        if content_type != String::new() {
            request = request.header(hyper::header::CONTENT_TYPE, content_type);
        }

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
        client.request(request.body(body).res()?).await.res()
    }

    #[cfg(feature = "serde")]
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        route: &str,
        data: Option<&str>,
    ) -> Result<T, DeltaError> {
        Delta::result(
            self.common(&format!("{}{}", self.url, route), method.into(), data)
                .await,
        )
        .await
    }
}

type Response = Result<hyper::Response<hyper::Body>, DeltaError>;

pub struct Method(MethodOption);

enum MethodOption {
    Post,
    Put,
    Patch,
    Get,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}

impl From<Method> for hyper::Method {
    fn from(value: Method) -> Self {
        match value.0 {
            MethodOption::Post => hyper::Method::POST,
            MethodOption::Put => hyper::Method::PUT,
            MethodOption::Patch => hyper::Method::PATCH,
            MethodOption::Get => hyper::Method::GET,
            MethodOption::Delete => hyper::Method::DELETE,
            MethodOption::Head => hyper::Method::HEAD,
            MethodOption::Options => hyper::Method::OPTIONS,
            MethodOption::Connect => hyper::Method::CONNECT,
            MethodOption::Trace => hyper::Method::TRACE,
        }
    }
}
impl Method {
    pub const POST: Method = Method(MethodOption::Post);
    pub const PUT: Method = Method(MethodOption::Put);
    pub const PATCH: Method = Method(MethodOption::Patch);
    pub const GET: Method = Method(MethodOption::Get);
    pub const DELETE: Method = Method(MethodOption::Delete);
    pub const HEAD: Method = Method(MethodOption::Head);
    pub const OPTIONS: Method = Method(MethodOption::Options);
    pub const CONNECT: Method = Method(MethodOption::Connect);
    pub const TRACE: Method = Method(MethodOption::Trace);
}
