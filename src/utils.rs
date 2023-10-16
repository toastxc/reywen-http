/// Converts a struct into query parameters by converting the struct to JSON (`serde_json`)
/// and makes them key-value pairs and ignores any `null` values and objects.
///
/// Arrays are not ignored.
#[cfg(feature = "serde")]
pub fn struct_to_url<T: serde::Serialize>(
    query: T,
    #[cfg(feature = "encoding")] encode: bool,
) -> String {
    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
        &serde_json::to_string(&query).unwrap_or_default(),
    )
    .unwrap_or_default()
    .into_iter()
    .enumerate()
    .filter_map(|(i, (key, value))| {
        let key_separator = if i == 0 { "?" } else { "&" };

        // arrays
        value.as_array().map_or_else(
            || {
                if value.is_object() || value.is_null() {
                    None
                } else {
                    #[cfg(feature = "encoding")]
                    let value = encode_value(&value, encode);

                    // standard value parse
                    Some(format!("{key_separator}{key}={value}"))
                }
            },
            |array| {
                let mut s = String::new();

                for (i, item) in array.iter().enumerate() {
                    let count_value = if i == 0 { key_separator } else { "&" };

                    #[cfg(feature = "encoding")]
                    let item = encode_value(item, encode);

                    s.push_str(&format!("{count_value}{key}[]={item}"));
                }

                Some(s)
            },
        )
    })
    .collect()
}

#[cfg(feature = "serde")]
#[cfg(feature = "encoding")]
#[must_use]
pub fn encode_value(value: &serde_json::Value, encode: bool) -> String {
    encode_str(&value.to_string().replace('"', ""), encode)
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
