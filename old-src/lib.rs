//pub mod driver;
//#[cfg(feature = "serde")]
//pub mod driver_serde;


pub mod new_request;
pub mod newtrait;
pub mod results;
//pub mod traits;
//pub mod utils;

#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub url: String,
    pub timeout: std::time::Duration,
    pub headers: hyper::header::HeaderMap,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
}

impl Delta {
    pub fn new() -> Self {
        Default::default()
    }
}
