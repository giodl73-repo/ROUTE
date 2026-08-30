//! Helper `t3_zone_bounds`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_bounds(zone_id: &str) -> Option<(f64, f64, f64, f64)> {
    match zone_id {
        "t3-great-lakes" => Some((37.0, 46.5, -92.0, -74.0)),
        "t3-southeast" => Some((25.0, 39.5, -91.5, -75.0)),
        "t3-texas-border" => Some((25.0, 34.5, -107.5, -93.0)),
        "t3-mountain-west" => Some((31.0, 49.5, -125.0, -102.0)),
        "t3-mid-south" => Some((29.0, 40.5, -96.5, -75.0)),
        _ => None,
    }
}
