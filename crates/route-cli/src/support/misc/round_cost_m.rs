//! Helper `round_cost_m`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn round_cost_m(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

