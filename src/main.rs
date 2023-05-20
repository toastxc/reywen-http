// Imprting needed libraries
use reywen_http::{driver2::Delta, results2::DeltaError};
// Generic JSON system
use serde_json::Value;

#[tokio::main]
async fn main() {
    // Defining HTTP Client, all setters start with 'set'
    let http = Delta::new()
        .set_url("https://api.hypixel.net")
        .set_timeout(10);

    // reqwesting data from bazaar, the serilization target for this API is that of the specified type as shown below
    let highpickle: Result<Value, DeltaError> =
        Delta::result(http.get("/skyblock/bazaar").await).await;

    match highpickle {
        Ok(_) => {
            println!("success owo");
        }
        Err(er) => {
            println!("{:#?}", er)
        }
    }
}





