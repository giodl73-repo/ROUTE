//! Helper `projection_fraction`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn projection_fraction(
    a_lat: f64,
    a_lon: f64,
    b_lat: f64,
    b_lon: f64,
    p_lat: f64,
    p_lon: f64,
) -> f64 {
    let lat0 = ((a_lat + b_lat + p_lat) / 3.0).to_radians();
    let ax = a_lon * lat0.cos();
    let ay = a_lat;
    let bx = b_lon * lat0.cos();
    let by = b_lat;
    let px = p_lon * lat0.cos();
    let py = p_lat;
    let dx = bx - ax;
    let dy = by - ay;
    let len2 = dx * dx + dy * dy;
    if len2 <= f64::EPSILON {
        0.0
    } else {
        ((px - ax) * dx + (py - ay) * dy) / len2
    }
}
