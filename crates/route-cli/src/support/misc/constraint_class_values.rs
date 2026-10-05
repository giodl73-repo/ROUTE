//! Helper `constraint_class_values`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn constraint_class_values(value: &str) -> Vec<String> {
    value
        .split([';', '|'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}
