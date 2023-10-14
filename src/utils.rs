#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "serde")]
pub fn struct_to_url<T: Serialize>(query: T, #[cfg(feature = "encoding")] encode: bool) -> String {
    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
        &serde_json::to_string(&query).unwrap_or_default(),
    )
    .unwrap_or_default()
    .into_iter()
    .filter_map(|(key, value)| {
        // arrays
        value.as_array().map_or_else(
            || {
                if value.is_object() || value.is_null() {
                    None
                } else {
                    #[cfg(feature = "encoding")]
                    let value = encode_value(&value, encode);

                    // standard value parse
                    Some(format!("{key}={value}"))
                }
            },
            |array| {
                let mut counter = 0;

                Some(
                    array
                        .iter()
                        .map(|item| {
                            let count_value = if counter == 0 { "" } else { "&" };
                            counter += 1;

                            #[cfg(feature = "encoding")]
                            let item = encode_value(item, encode);

                            format!("{count_value}{key}[]={item}")
                        })
                        .collect(),
                )

                // blacklist - non supported types
            },
        )
    })
    .collect::<Vec<String>>()
    .into_iter()
    .enumerate()
    .map(|(index, item)| (if index == 0 { "?" } else { "&" }).to_string() + &item)
    .collect::<String>()
    .replace('"', "")
}

#[cfg(feature = "serde")]
#[cfg(feature = "encoding")]
#[must_use]
pub fn encode_value(value: &serde_json::Value, encode: bool) -> String {
    encode_str(&value.to_string(), encode)
}

#[cfg(feature = "encoding")]
#[cfg(feature = "serde")]
#[must_use]
pub fn encode_str(value: &str, encode: bool) -> String {
    if encode {
        urlencoding::encode(value).into_owned()
    } else {
        value.to_owned()
    }
}
