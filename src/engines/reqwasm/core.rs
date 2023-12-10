use crate::engines::reqwasm::{
    results::{Error, Result},
    Body, Reqwasm,
};
use js_sys::{Object, Reflect, Uint8Array};
use reqwasm::http::{Headers, Method, Request};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::ReadableStreamDefaultReader;

impl Reqwasm {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Reqwasm {
    pub fn set_url(&mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self.to_owned()
    }
    pub fn set_user_agent(&mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self.to_owned()
    }
    pub fn set_content_type(&mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self.to_owned()
    }
    pub fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<Self> {
        self.headers.append(&key.into(), &value.into());
        Ok(self.to_owned())
    }
    pub fn set_headers(&mut self, headers: Headers) -> Self {
        self.headers = headers;
        self.to_owned()
    }
}

impl Reqwasm {
    #[cfg(feature = "serde")]
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<T> {
        self.common(method.into(), path.into(), Reqwasm::from_bytes(data.into())).await?.serde_switch()
    }

    pub fn to_bytes(input: Option<JsValue>) -> Option<Vec<u8>> {
        let input = input?;
        let input = input
            .as_string()
            .expect("Value could not be turned into a String from JS");
        input.into_bytes().into()
    }

    pub fn from_bytes(input: Option<Vec<u8>>) -> Option<JsValue> {
        JsValue::from_str(&String::from_utf8(input?).expect("Bytes are not uft8")).into()
    }

    pub async fn request_raw(
        &self,
        method: impl Into<Method>,
        path: impl Into<String>,
        data: impl Into<Option<Vec<u8>>>,
    ) -> Result<Vec<u8>> {
       let a =  self.common(
            method.into(),
            format!("{}{}", self.url, path.into()),
            Reqwasm::from_bytes(data.into()),
        )
        .await?;
        Ok(a.body.unwrap_or_default())
    }

    pub async fn common(
        &self,
        method: Method,
        url: String,
        data: Option<wasm_bindgen::JsValue>,
    ) -> Result<Body> {
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

        Ok(Body {
            body,
            status: unsafe { std::num::NonZeroU16::new_unchecked(status) },
        })
    }
}

#[cfg(feature = "serde")]
impl Body {
    pub fn serde_switch<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        match (self.body, self.status.get()) {
            (Some(data), 200) => Ok(serde_json::from_slice(&data)?),
            (None, 200) | (_, 204) => Ok(serde_json::from_value(serde_json::Value::Null)?),
            _ => Err(Error::StatusCode(self.status)),
        }
    }
}
