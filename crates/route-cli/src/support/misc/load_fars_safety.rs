//! Helper `load_fars_safety`.
#[allow(unused_imports)]
use crate::*;

/// Load FARS 2022 fatal crash rates by route from data/cache/fars_2022_routes.csv.
/// Columns: route_id, fatal_count, fatal_rate_per_100mvmt
/// Returns route_id -> crash_rate_per_100M_VMT.
pub(crate) fn load_fars_safety() -> std::collections::HashMap<String, f32> {
    let path = std::path::Path::new("data/cache/fars_2022_routes.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 3 {
            continue;
        }
        let route_id = result[0].to_string();
        let rate: f32 = result[2].parse().unwrap_or(0.0);
        map.insert(route_id, rate);
    }
    map
}

