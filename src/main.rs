use reywen_http::{driver::Delta, results::DeltaError};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let http = Delta::new()
        .set_url("https://api.hypixel.net")
        .set_timeout(10);

    let highpickle: Result<Value, DeltaError> =
        Delta::result(http.get("/skyblock/bazaar").await).await;

    println!("{:#?}", highpickle)
}
