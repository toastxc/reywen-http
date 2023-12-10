pub mod core;
pub mod results;
pub mod tests;

// re-exports
pub use reqwasm::http::{Headers, Method};

#[derive(Debug, Clone)]
pub struct Body {
    pub body: Option<Vec<u8>>,
    pub status: std::num::NonZeroU16,
}

#[derive(Debug, Default)]
pub struct Reqwasm {
    pub url: String,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub headers: Headers,
}

impl Reqwasm {
    pub fn header_delete(&mut self, key: impl Into<String>) {
        self.headers.delete(&key.into())
    }
}

impl Clone for Reqwasm {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            user_agent: self.user_agent.clone(),
            content_type: self.content_type.clone(),
            headers: header_clone(self.headers.entries()),
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.url = source.url.clone();
        self.user_agent = source.user_agent.clone();
        self.content_type = source.content_type.clone();
        self.headers = header_clone(source.headers.entries());
    }
}

fn header_clone(input: impl Iterator<Item = (String, String)>) -> Headers {
    let new_headers = Headers::new();
    input.for_each(|(key, value)| {
        new_headers.append(&key, &value);
    });
    new_headers
}

impl Default for Body {
    fn default() -> Self {
        Self {
            body: None,
            status: unsafe { std::num::NonZeroU16::new_unchecked(200) },
        }
    }
}
