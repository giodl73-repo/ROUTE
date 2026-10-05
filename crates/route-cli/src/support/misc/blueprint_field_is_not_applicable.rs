//! Helper `blueprint_field_is_not_applicable`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_field_is_not_applicable(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "n/a" | "not_applicable" | "none"
    )
}
