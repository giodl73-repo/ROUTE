//! Helper `distance_to_geo_segment_miles`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn distance_to_geo_segment_miles(
    a_lat: f64,
    a_lon: f64,
    b_lat: f64,
    b_lon: f64,
    p_lat: f64,
    p_lon: f64,
) -> f64 {
    let t = projection_fraction(a_lat, a_lon, b_lat, b_lon, p_lat, p_lon).clamp(0.0, 1.0);
    let lat = a_lat + (b_lat - a_lat) * t;
    let lon = a_lon + (b_lon - a_lon) * t;
    geo_distance_miles(lat, lon, p_lat, p_lon)
}
