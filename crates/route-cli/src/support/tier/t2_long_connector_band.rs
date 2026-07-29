//! Helper `t2_long_connector_band`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_long_connector_band(schematic_length_px: f64) -> &'static str {
    if schematic_length_px >= 1200.0 {
        "severe-long-connector"
    } else if schematic_length_px >= 900.0 {
        "high-long-connector"
    } else {
        "moderate-long-connector"
    }
}

