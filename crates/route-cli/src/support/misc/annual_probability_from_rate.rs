//! Helper `annual_probability_from_rate`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn annual_probability_from_rate(rate: f64) -> f64 {
    if rate <= 0.0 {
        0.0
    } else {
        1.0 - (-rate).exp()
    }
}
