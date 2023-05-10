# reywen-http


### Why another HTTP lib?
Originally I maintained my own HTTP library within a project for Revolt.chat, but it became too large and is now in it's own repository. That said the library can be easily used by anyone for any API!


### Example Using HighPixel API

#### GET Request
As shown below the library can be used without much prior setup or configuration, and runs asyncronously.
```rust
// Imprting needed libraries
use reywen_http::{driver::Delta, results::DeltaError};
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
        Ok(data) => {
            println!("{:#?}\nSuccess! Latest bazaar data :3", data)
        }
        Err(er) => {
            println!("{:#?}", er)
        }
    }
}
```

#### More complex request
For use in large projects ideally methods are seperated into their own functions (or files) and are connected in a large `impl`
Here an example for using reywen-http for revolt.chat, for methods that can accept a data body (non GET) the data is represented by an Option as shown below
```rust
use reywen_http::{driver::Delta, results::DeltaError};
use serde::{Deserialize, Serialize};

pub async fn channel_edit(
    http: &Delta,
    channel: &str,
    edit_data: &DataEditChannel,
) -> Result<Channel, DeltaError> {
    let data = serde_json::to_string(edit_data).unwrap();
    Delta::result(
        http.patch(&format!("/channels/{channel}"), Some(&data))
            .await,
    )
    .await
}


```
