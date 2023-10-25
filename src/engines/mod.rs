#[cfg(feature = "hyper_engine")]
pub mod hyper;

#[cfg(feature = "reqwasm_engine")]
pub mod reqwasm;

#[cfg(feature = "reqwest_engine")]
pub mod reqwest;

pub trait Setter {
    #[must_use]
    fn set_url(&mut self, url: impl Into<String>) -> Self;
    #[must_use]
    fn set_user_agent(&mut self, user_agent: impl Into<String>) -> Self;
    #[must_use]
    fn set_content_type(&mut self, content_type: impl Into<String>) -> Self;
}

