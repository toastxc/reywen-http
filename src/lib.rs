pub mod hyper_driver;
pub mod request;
pub mod results;

pub mod utils;

pub mod engines;

#[cfg(feature = "serde")]
pub mod serde;

use hyper::header;
#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub url: String,
    pub timeout: std::time::Duration,
    pub headers: header::HeaderMap,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
}

impl Delta {
    pub fn new() -> Self {
        Default::default()
    }
}
