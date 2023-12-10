#[cfg(test)]
mod tests {
    use crate::engines::reqwest::Reqwest;
    use reqwest::Method;

    #[tokio::test]
    async fn request_basic() {
        assert!(Reqwest::new()
            .request_raw(Method::GET, "https://api.revolt.chat", None)
            .await
            .is_ok());
    }
}
