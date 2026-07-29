//! Helper `high_or_medium`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn high_or_medium(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "high" | "medium" | "met"
    )
}

