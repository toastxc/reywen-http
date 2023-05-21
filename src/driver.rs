use hyper::{
    header::USER_AGENT,
    http::{HeaderName, HeaderValue},
    Client, Request,
};
use hyper_tls::HttpsConnector;

#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub url: String,
    pub timeout: std::time::Duration,
    pub headers: hyper::header::HeaderMap,
    pub user_agent: Option<String>,
}

impl Delta {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

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

    pub fn set_headers(&mut self, headers: Vec<(&str, &str)>) -> Self {
        self.remove_headers();
        self.add_headers(headers);
        self.to_owned()
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> Self {
        let (key, value) = Self::kv_parse(key, value);
        self.headers.insert(key, value);

        self.to_owned()
    }

    pub fn add_headers(&mut self, headers: Vec<(&str, &str)>) -> Self {
        for (key, value) in headers.clone() {
            let (key, value) = Self::kv_parse(key, value);
            self.headers.append(key, value);
        }
        self.to_owned()
    }
    pub fn remove_headers(&mut self) -> Self {
        self.headers = reqwest::header::HeaderMap::new();
        self.to_owned()
    }

    pub async fn get(&self, route: &str) -> Response {
        self.common(&format!("{}{}", self.url, route), hyper::Method::GET, None)
            .await
    }

    pub async fn post(&self, route: &str, data: Option<&str>) -> Response {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::POST,
            data,
        )
        .await
    }

    pub async fn put(&self, route: &str, data: Option<&str>) -> Response {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::PUT,
            data,
        )
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

    pub fn kv_parse(key: &str, value: &str) -> (HeaderName, HeaderValue) {
        (
            hyper::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
            hyper::header::HeaderValue::from_str(value).unwrap(),
        )
    }

    async fn common(&self, url: &str, method: hyper::Method, input_data: Option<&str>) -> Response {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut headers = hyper::HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(
                &self
                    .clone()
                    .user_agent
                    .unwrap_or(String::from("Reywen-HTTP/10.0 (async-tokio-runtime)")),
            )
            .expect("invalid user agent!"),
        );

        let (body, content_type) = match input_data {
            Some(data) => (hyper::body::Body::from(data.to_owned()), "application/json"),
            None => (hyper::Body::empty(), "text/plain"),
        };

        for (key, value) in self.headers.clone() {
            if let Some(key) = key {
                headers.insert(key, value);
            };
        }

        let mut request = Request::builder()
            .method(method)
            .uri(url)
            .header(hyper::header::CONTENT_TYPE, content_type);

        request.headers_mut().unwrap().extend(headers.into_iter());

        client.request(request.body(body).unwrap()).await
    }
}
pub type Response = Result<hyper::Response<hyper::Body>, hyper::Error>;
