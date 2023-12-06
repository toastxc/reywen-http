#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

pub mod engines;
pub mod utils;


pub const USER_AGENT: &str = "Reywen-HTTP/10.0 (async-tokio-runtime)";
