#[cfg(test)]
mod tests {
    use crate::engines::hyper::Hyper;
    use hyper::Method;
    use serde_json::Value;

    #[tokio::test]
    async fn hyper_req_raw() {
        assert!(Hyper::new()
            .request_raw(Method::GET, "https://api.revolt.chat", None)
            .await
            .is_ok());
    }
    #[tokio::test]
    async fn hyper_req() {
        assert!(Hyper::new()
            .request::<Value>(Method::GET, "https://repo.toastxc.xyz/empty.json", None)
            .await
            .is_ok());
    }
}
