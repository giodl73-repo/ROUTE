//! Helper `sort_stops_for_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn sort_stops_for_route(stops: &mut [&StopCandidateRow]) {
    if stops.len() < 2 {
        return;
    }
    let coords = stops
        .iter()
        .filter_map(|row| Some((parse_coord(&row.lat)?, parse_coord(&row.lon)?)))
        .collect::<Vec<_>>();
    if coords.len() < 2 {
        stops.sort_by(|a, b| a.name.cmp(&b.name));
        return;
    }
    let (min_lat, max_lat) = coords
        .iter()
        .map(|(lat, _)| *lat)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), value| {
            (min.min(value), max.max(value))
        });
    let (min_lon, max_lon) = coords
        .iter()
        .map(|(_, lon)| *lon)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), value| {
            (min.min(value), max.max(value))
        });
    let lat_span = max_lat - min_lat;
    let lon_span = max_lon - min_lon;
    if lat_span >= lon_span {
        stops.sort_by(|a, b| coord_or_default(&a.lat).total_cmp(&coord_or_default(&b.lat)));
    } else {
        stops.sort_by(|a, b| coord_or_default(&a.lon).total_cmp(&coord_or_default(&b.lon)));
    }
}
