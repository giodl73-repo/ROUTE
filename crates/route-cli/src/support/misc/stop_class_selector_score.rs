//! Helper `stop_class_selector_score`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_class_selector_score(value: &str) -> u16 {
    match value.trim().to_ascii_uppercase().as_str() {
        "S1" => 5,
        "S2" => 4,
        "S3" => 3,
        "S4" => 2,
        "S5" => 1,
        _ => 0,
    }
}
