
use crate::{results::DeltaError, Delta};
use hyper::{
    header::{self, USER_AGENT},
    http::{HeaderName, HeaderValue},
    Body, Request,
};

use hyper_tls::HttpsConnector;

use hyper::header::CONTENT_TYPE;
use hyper::Client;

#[cfg(feature = "serde")]
use serde;

#[derive(Debug, Clone, Default)]
pub struct Delta2 {
    pub url: String,
    pub timeout: std::time::Duration,
    pub headers: hyper::header::HeaderMap,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
}

impl Delta2 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Delta2 {
    #[cfg(feature = "serde")]
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        route: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<T, DeltaError> {
        // Delta::result(
        //     self.common(&format!("{}{}", self.url, route), method.into(), Some(data))
        //         .await,
        // )
        // .await
        todo!()
    }
}

type Response = Result<hyper::Response<hyper::Body>, DeltaError>;

impl Delta2 {
    pub async fn common(
        &self,
        method: hyper::Method,
        url: impl Into<String>,
        body: Option<impl Into<Vec<u8>>>,
    ) -> Response {

        // HTTPS client
        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        let mut request = Request::builder().method(method).uri(url.into());

        // headers
        let mut headers = self.to_owned().headers;
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(
                self.user_agent
                    .as_deref()
                    .unwrap_or("Reywen-HTTP/10.0 (async-tokio-runtime)"),
            )?,
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

        // Request
        Ok(client
            .request(request.body(if let Some(body_vec) = body {
                Body::from(body_vec.into())
            } else {
                Body::empty()
            })?)
            .await?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_headers() {
        let mut delta2 = Delta2::default();
        delta2.url = String::from("https://g.co");
        delta2.headers.insert("hewo", "hew".try_into().unwrap());
        delta2.headers.insert("aaa", "hew".try_into().unwrap());

        // let a: Result<(), DeltaError> = delta2.request(Method::GET, "ROUTE", vec![0]).await;

        let a = delta2
            .common(Method::GET.into(), "https://g.co", Some(vec![0]))
            .await;

        println!("{:?}", a);
    }
}


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
