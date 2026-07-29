//! Helper `t3_stop_in_zone`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_stop_in_zone(stop: &StopCandidateRow, zone_id: &str) -> bool {
    let Some((min_lat, max_lat, min_lon, max_lon)) = t3_zone_bounds(zone_id) else {
        return true;
    };
    let Some(lat) = parse_coord(&stop.lat) else {
        return false;
    };
    let Some(lon) = parse_coord(&stop.lon) else {
        return false;
    };
    (min_lat..=max_lat).contains(&lat) && (min_lon..=max_lon).contains(&lon)
}

