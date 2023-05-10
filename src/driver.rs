use reqwest::Response;

use crate::header::HeaderData;

#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub url: String,
    pub timeout: std::time::Duration,
    pub headers: reqwest::header::HeaderMap,
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
        let header = HeaderData::new(key, value).unwrap().as_header_value();
        self.headers.append(header.0, header.1);
        self.to_owned()
    }

    pub fn add_headers(&mut self, headers: Vec<(&str, &str)>) -> Self {
        for (key, value) in headers.clone() {
            let (newkey, newvalue) = HeaderData::new(key, value).unwrap().as_header_value();
            self.headers.append(newkey, newvalue);
        }
        self.to_owned()
    }
    pub fn remove_headers(&mut self) -> Self {
        self.headers = reqwest::header::HeaderMap::new();
        self.to_owned()
    }

    pub async fn get(&self, route: &str) -> Result<Response, reqwest::Error> {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::GET,
            None,
        )
        .await
    }
    pub async fn post(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::POST,
            data,
        )
        .await
    }

    pub async fn put(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::PUT,
            data,
        )
        .await
    }

    pub async fn delete(
        &self,
        route: &str,
        data: Option<&str>,
    ) -> Result<Response, reqwest::Error> {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::DELETE,
            data,
        )
        .await
    }
    pub async fn patch(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        self.common(
            &format!("{}{}", self.url, route),
            reqwest::Method::PATCH,
            data,
        )
        .await
    }

    async fn common(
        &self,
        url: &str,
        method: reqwest::Method,
        data: Option<&str>,
    ) -> Result<Response, reqwest::Error> {
        let builder = reqwest::ClientBuilder::new()
            .timeout(self.timeout)
            .user_agent(
                self.clone()
                    .user_agent
                    .unwrap_or(String::from("Reywen-HTTP/10.0 (async-tokio-runtime)")),
            );

        // client constructor
        let mut client = builder.build().unwrap().request(method, url);

        // headers
        if !self.headers.is_empty() {
            client = client.headers(self.headers.clone());
        }

        // data body
        if let Some(json) = data {
            let json = json.to_string();
            client = client.header("Content-Type", "application/json").body(json);
        };

        // send request
        client.send().await
    }
}
