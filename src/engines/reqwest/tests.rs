#[cfg(test)]
mod tests {
    use super::Reqwest;
    use crate::engines::reqwest::core::Reqwest;
    use reqwest::Method;

    #[tokio::test]
    async fn request_basic() {
        assert!(Reqwest::new()
            .request_raw(Method::GET, "https://api.revolt.chat", None)
            .await
            .is_ok());
    }
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn request_basic() {
    assert!(Reqwest::new()
        .request_raw(Method::GET, "https://api.revolt.chat", None)
        .await
        .is_ok());
}
