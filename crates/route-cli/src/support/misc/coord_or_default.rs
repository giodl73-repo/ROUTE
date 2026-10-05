//! Helper `coord_or_default`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn coord_or_default(value: &str) -> f64 {
    parse_coord(value).unwrap_or(0.0)
}
