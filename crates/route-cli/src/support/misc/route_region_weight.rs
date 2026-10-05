//! Helper `route_region_weight`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_region_weight(route_miles: f64) -> i32 {
    route_miles.round().clamp(1.0, i32::MAX as f64) as i32
}
