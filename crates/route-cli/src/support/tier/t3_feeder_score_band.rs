//! Helper `t3_feeder_score_band`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_feeder_score_band(current_score: f64) -> &'static str {
    if current_score >= 29.0 {
        "near-threshold-feeder"
    } else if current_score >= 25.0 {
        "low-threshold-feeder"
    } else {
        "out-of-band-feeder"
    }
}

