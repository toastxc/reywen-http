#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

pub mod engines;
pub mod results;
pub mod utils;

use crate::results::DeltaError;
use hyper::header;

pub type DeltaResponse<HttpE, HN, HV> = Result<DeltaBody, DeltaError<HttpE, HN, HV>>;

#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub url: String,
    pub timeout: std::time::Duration,
    pub headers: header::HeaderMap,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeltaBody {
    pub body: Option<Vec<u8>>,
    pub status: StatusCode,
}

#[derive(Debug, Clone)]
pub struct StatusCode(pub std::num::NonZeroU16);
impl StatusCode {
    #[must_use]
    pub fn as_u16(&self) -> u16 {
        self.0.into()
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Delta {
    /// Creates a new [`Delta`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
