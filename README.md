# Reywen-HTTP


### Why another HTTP lib?
Originally I maintained my own HTTP library within a project for Revolt.chat, but it became too large and is now in it's own repository. That said the library can be easily used by anyone for any API!

### Features
- built-in serde support
- can use a variety of HTTP engines
- WASM support
- Tokio async

### Example Using Hypixel API
As shown below the library can be used without much prior setup or configuration, and runs asynchronously.

This example uses Hyper as its backend, however there are many different HTTP engines available for use. All of them implement the same Request/ReqRaw syntax
```rust
use crate::engines::hyper::{Error, Hyper};
use hyper::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ExampleData {
    field1: String,
}

impl From<ExampleData> for Option<Vec<u8>> {
    fn from(value: ExampleData) -> Self {
        todo!()
    }
}
pub async fn hypixel_example() -> Result<(), Error> {
    // define client, fields within the client declaration are global and will apply to all requests
    // unless overwritten
    let client = Hyper::new().set_url("https://api.hypixel.net");

    // request requires serde and will deserialize data based on the T input type
    println!(
        "{}",
        client
            .request::<Value>(Method::GET, "/skyblock/bazaar", None)
            .await?
    );

    // request raw will return a byte array for as the response
    client
        .request_raw(Method::GET, "/skyblock/bazaar", None)
        .await?;

    // data sent over request or request_raw must be of type Vec<u8>, String or any type that can be
    // converted to those types
    client
        .clone()
        .set_url("example.com")
        .request_raw(Method::POST, "", ExampleData::default())
        .await?;
    Ok(())
}
```
