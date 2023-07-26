#[cfg(feature = "serde")]
use serde::Serialize;

/// converts the data of a structure into url query parameters
/// this function does not support arrays or objects  
#[cfg(feature = "serde")]
pub fn struct_to_url_vlod<T: Serialize>(query: T) -> String {
    let mut iter = Vec::new();

    let json_str = serde_json::to_string(&query).unwrap_or_default();
    let json_obj = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&json_str)
        .unwrap_or_default();

    for (key, value) in json_obj {
        // arrays and objects are not supported in a URL query, and as such cannot be imported
        if value.is_object() || value.is_array() | value.is_null() {
            continue;
        }
        iter.push(format!("{key}={}", urlencoding::encode(&value.to_string())));
    }

    if iter.is_empty() {
        return String::new();
    };

    let mut str = String::new();

    for (num, item) in iter.iter().enumerate() {
        let temp = match (num, item) {
            (0, x) => format!("?{x}"),
            (_, x) => format!("&{x}"),
        };
        str += &temp;
    }
    str
}

/// converts the data of a structure into url query parameters
/// this function does not support objects  
#[cfg(feature = "serde")]
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

#[tokio::test]
async fn test_url() {
    #[derive(Debug, Serialize, Default, Clone)]
    pub struct DataStruct {
        string: String,
        object: DataObject,
        array: Vec<String>,
    }
    #[derive(Debug, Serialize, Default, Clone)]
    pub struct DataObject {
        s1: String,
        s2: String,
    }

    let url = "example.com";
    let data = DataStruct {
        string: String::from("haiii"),
        array: vec!["item".to_string(), "in".to_string(), "vec".to_string()],
        ..Default::default()
    };
    // testing different implementations for url encoding
    println!("ORIGINAL: {url}, {:?}", data);
    println!("TOAST ENCODED: {url}{}", struct_to_url(&data, false));
    println!("TOAST ENCODED ENABLED: {url}{}", struct_to_url(&data, true));
    println!("VLOD ENCODED: {url}{}", struct_to_url_vlod(&data));
}

pub fn if_false(t: &bool) -> bool {
    !t
}
#[cfg(feature = "serde")]
pub fn encode_value(value: &serde_json::Value, encode: bool) -> String {
    encode_str(&value.to_string(), encode)
}

pub fn encode_str(value: &str, encode: bool) -> String {
    if encode {
        urlencoding::encode(value).into_owned()
    } else {
        value.to_owned()
    }
}
