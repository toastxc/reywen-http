pub mod core;
pub mod results;
pub mod tests;

// re-exports
use crate::engines::hyper::results::Error;
pub use hyper::HeaderMap;
pub use hyper::Method;
use hyper::StatusCode;

// structures
pub struct Body {
    pub body: Option<Vec<u8>>,
    pub status: StatusCode,
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, Default)]
pub struct Hyper {
    pub url: String,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub headers: hyper::HeaderMap,
}
