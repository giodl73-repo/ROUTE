//! Helper `selector_signal_score`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn selector_signal_score(value: &str) -> u16 {
    match value.trim().to_ascii_lowercase().as_str() {
        "high" | "met" | "required" => 3,
        "medium" | "planned" | "review_needed" => 2,
        "low" => 1,
        _ => 0,
    }
}
