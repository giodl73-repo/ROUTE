//! Helper `json_value_string`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn json_value_string(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

