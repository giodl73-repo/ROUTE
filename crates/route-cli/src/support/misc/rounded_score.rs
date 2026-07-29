//! Helper `rounded_score`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn rounded_score(score: f64) -> f64 {
    (score * 10.0).round() / 10.0
}

