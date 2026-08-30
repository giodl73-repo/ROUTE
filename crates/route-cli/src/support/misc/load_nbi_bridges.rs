//! Helper `load_nbi_bridges`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_nbi_bridges() -> std::collections::HashMap<String, NbiBridgeRecord> {
    let path = std::path::Path::new("data/cache/nbi_bridges.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut totals: std::collections::HashMap<String, (u32, f32, f32)> =
        std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 5 {
            continue;
        }
        let route_id = normalise_designation(result[0].trim());
        if route_id.is_empty() {
            continue;
        }
        let bridge_count: u32 = result[1].parse().unwrap_or(0);
        let pct: f32 = result[3].parse().unwrap_or(0.0);
        let year: f32 = result[4].parse().unwrap_or(1970.0);
        let poor_count = pct * bridge_count as f32;
        let year_sum = year * bridge_count as f32;
        let entry = totals.entry(route_id).or_insert((0, 0.0, 0.0));
        entry.0 += bridge_count;
        entry.1 += poor_count;
        entry.2 += year_sum;
    }
    let mut map = std::collections::HashMap::new();
    for (route_id, (bridge_count, poor_count, year_sum)) in totals {
        let denom = bridge_count.max(1) as f32;
        map.insert(
            route_id,
            NbiBridgeRecord {
                pct_bridges_poor: poor_count / denom,
                mean_year_built: year_sum / denom,
                bridge_count,
            },
        );
    }
    map
}
