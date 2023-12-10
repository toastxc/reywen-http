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
