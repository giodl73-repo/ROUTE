//! Helper `t2_label_density_band`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_label_density_band(label_density_per_100px: f64) -> &'static str {
    if label_density_per_100px >= 1.25 {
        "severe-label-density"
    } else if label_density_per_100px >= 1.10 {
        "high-label-density"
    } else {
        "moderate-label-density"
    }
}
