pub mod core;
pub mod results;
pub mod tests;

// re-exports
pub use crate::engines::reqwest::results::Error;
pub use reqwest::header::HeaderMap;
pub use reqwest::header::HeaderName;
pub use reqwest::Method;
pub use reqwest::StatusCode;

#[derive(Debug, Clone, Default)]
pub struct Reqwest {
    pub url: String,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub headers: reqwest::header::HeaderMap,
}

#[derive(Debug, Clone)]
pub struct ReqwestBody {
    pub body: Option<Vec<u8>>,
    pub status: StatusCode,
}
