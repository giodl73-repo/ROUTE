//! Helper `geo_distance_miles`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn geo_distance_miles(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    let earth_radius_miles = 3958.8_f64;
    let dlat = (b_lat - a_lat).to_radians();
    let dlon = (b_lon - a_lon).to_radians();
    let h = (dlat / 2.0).sin().powi(2)
        + a_lat.to_radians().cos() * b_lat.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * earth_radius_miles * h.sqrt().asin() * 1.18
}
