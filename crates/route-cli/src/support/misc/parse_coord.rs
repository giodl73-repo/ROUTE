//! Helper `parse_coord`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_coord(value: &str) -> Option<f64> {
    value
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

