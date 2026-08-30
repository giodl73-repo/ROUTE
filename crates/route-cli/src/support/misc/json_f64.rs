//! Helper `json_f64`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn json_f64(
    attrs: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<f64> {
    attrs.get(key).and_then(|value| {
        value
            .as_f64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
    })
}
