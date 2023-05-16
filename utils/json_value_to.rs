use std::str::FromStr;

use deno_core::serde_json::Value;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn value_to_hasmap(value: Value) -> Option<HeaderMap> {
    if let Some(map) = value.as_object() {
        let mut result = HeaderMap::new();
        for (key, value) in map {
            result.insert(
                HeaderName::from_str(key.clone().as_str()).unwrap(),
                HeaderValue::from_str(&value.clone().to_string()).unwrap(),
            );
        }
        Some(result)
    } else {
        None
    }
}
