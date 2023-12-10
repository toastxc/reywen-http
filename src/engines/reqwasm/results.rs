use std::fmt::{Debug, Formatter};

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

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Error::Engine(a) => format!("{:?}", a),
            Error::Js(_) => format!("JsError Debug print is not supported"),
            Error::JsConversion(a) => format!("{:?}", a),
            #[cfg(feature = "serde")]
            Error::Serde(a) => format!("{:?}", a),

            Error::StatusCode(a) => format!("{:?}", a),
        };
        write!(f, "{data}")
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
