use js_sys::{Object, Reflect, Uint8Array};
use reqwasm::http::{Headers, Method, Request};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::ReadableStreamDefaultReader;

#[derive(Debug, Clone)]
pub struct ReqwasmBody {
    pub body: Option<Vec<u8>>,
    pub status: std::num::NonZeroU16,
}

#[cfg(feature = "serde")]
impl ReqwasmBody {
    pub fn serde_switch<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        match (self.body, self.status.get()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(Error::StatusCode(self.status)),
        }
    }
}

impl Default for ReqwasmBody {
    fn default() -> Self {
        Self {
            body: None,
            status: unsafe { std::num::NonZeroU16::new_unchecked(200) },
        }
    }
}

pub enum Error {
    Engine(reqwasm::Error),
    Js(wasm_bindgen::JsError),

    /// Conversion of a JS value when processing the request.
    /// Likely a bug in the library.
    JsConversion(wasm_bindgen::JsValue),

    #[cfg(feature = "serde")]
    Serde(serde_json::Error),
    StatusCode(std::num::NonZeroU16),
}

impl From<reqwasm::Error> for Error {
    fn from(value: reqwasm::Error) -> Self {
        Self::Engine(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<wasm_bindgen::JsError> for Error {
    fn from(value: wasm_bindgen::JsError) -> Self {
        Self::Js(value)
    }
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self::JsConversion(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct Reqwasm {
    pub url: String,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub headers: Headers,
}

impl Reqwasm {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Reqwasm {
    #[cfg(feature = "serde")]
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<wasm_bindgen::JsValue>>,
    ) -> Result<T> {
        self.request_raw(method, path, data).await?.serde_switch()
    }

    pub async fn request_raw(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<wasm_bindgen::JsValue>>,
    ) -> Result<ReqwasmBody> {
        self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            data.into(),
        )
        .await
    }

    pub async fn common(
        &self,
        method: Method,
        url: String,
        data: Option<wasm_bindgen::JsValue>,
    ) -> Result<ReqwasmBody> {
        let mut request = Request::new(&url).body(data).method(method).header(
            "User-agent",
            self.user_agent.as_deref().unwrap_or(crate::USER_AGENT),
        );

        if let Some(content_type) = self.content_type.as_ref() {
            request = request.header("Content-Type", content_type);
        }

        for (key, value) in self.headers.entries() {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;

        let status = response.status();

        let body = match response.body() {
            Some(body) => {
                let reader: ReadableStreamDefaultReader =
                    body.get_reader().dyn_into().map_err(JsValue::from)?;

                let result: Object = JsFuture::from(reader.read()).await?.dyn_into()?;

                let result: Uint8Array =
                    Reflect::get(&result, &JsValue::from_str("value"))?.dyn_into()?;

                Some(result.to_vec())
            }
            None => None,
        };

        Ok(ReqwasmBody {
            body,
            status: unsafe { std::num::NonZeroU16::new_unchecked(status) },
        })
    }
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn reqwasm_engine() {
    assert!(Reqwasm::new()
        .request_raw(Method::GET, "https://api.revolt.chat", None)
        .await
        .is_ok());
}
