//! Helper `json_string`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn json_string(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> String {
    attrs
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}
