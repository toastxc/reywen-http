#[cfg(feature = "serde")]
use serde::Serialize;


/// converts the data of a structure into url query parameters
/// this function does not support objects  
#[cfg(feature = "serde")]
#[cfg(not(feature = "encoding"))]
pub fn struct_to_url<T: Serialize>(query: T) -> String {
    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
        &serde_json::to_string(&query).unwrap_or_default(),
    )
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(key, value)| {
            // arrays
            if let Some(array) = value.as_array() {
                let mut counter = 0;

                Some(
                    array
                        .iter()
                        .map(|item| {
                            let count_value = if counter == 0 { "" } else { "&" };
                            counter += 1;
                            format!("{}{key}[]={}", count_value, item, )
                        })
                        .collect(),
                )

                // blacklist - non supported types
            } else if value.is_object() | value.is_null() {
                None
            } else {
                // standard value parse
                Some(format!("{key}={}", value))
            }
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
pub fn struct_to_url<T: Serialize>(query: T, encode: bool) -> String {
    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
        &serde_json::to_string(&query).unwrap_or_default(),
    )
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(key, value)| {
            // arrays
            if let Some(array) = value.as_array() {
                let mut counter = 0;

                Some(
                    array
                        .iter()
                        .map(|item| {
                            let count_value = if counter == 0 { "" } else { "&" };
                            counter += 1;
                            format!("{}{key}[]={}", count_value, encode_value(item, encode))
                        })
                        .collect(),
                )

                // blacklist - non supported types
            } else if value.is_object() | value.is_null() {
                None
            } else {
                // standard value parse
                Some(format!("{key}={}", encode_value(&value, encode)))
            }
        })
        .collect::<Vec<String>>()
        .into_iter()
        .enumerate()
        .map(|(index, item)| (if index == 0 { "?" } else { "&" }).to_string() + &item)
        .collect::<String>()
        .replace('"', "")
}

pub fn if_false(t: &bool) -> bool {
    !t
}

#[cfg(feature = "serde")]
#[cfg(feature = "encoding")]
pub fn encode_value(value: &serde_json::Value, encode: bool) -> String {
    encode_str(&value.to_string(), encode)
}


#[cfg(feature = "encoding")]
#[cfg(feature = "serde")]
pub fn encode_str(value: &str, encode: bool) -> String {
    if encode {
        urlencoding::encode(value).into_owned()
    } else {
        value.to_owned()
    }
}
