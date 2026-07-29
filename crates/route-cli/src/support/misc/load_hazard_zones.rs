//! Helper `load_hazard_zones`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_hazard_zones() -> std::collections::HashMap<String, HazardZone> {
    let path = std::path::Path::new("data/hazard_zones.csv");
    let manifest_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/hazard_zones.csv");
    let path = if path.exists() {
        path.to_path_buf()
    } else {
        manifest_path
    };
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .flexible(true)
        .has_headers(false)
        .from_path(path)
    else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 4 {
            continue;
        }
        let route_raw = result[0].trim();
        if route_raw.eq_ignore_ascii_case("route_id") {
            continue;
        }
        // Extract base route: "I-5 (CA Siskiyou)" -> "I5"
        let id: String = route_raw
            .split_whitespace()
            .next()
            .unwrap_or("")
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_uppercase();
        let wf: f32 = result[1].parse().unwrap_or(0.0);
        let tor: f32 = result[2].parse().unwrap_or(0.0);
        let seis: f32 = result[3].parse().unwrap_or(0.0);
        // Take MAX for corridors spanning multiple segment entries
        let entry = map.entry(id).or_insert(HazardZone {
            wildfire: 0.0,
            tornado: 0.0,
            seismic: 0.0,
        });
        if wf > entry.wildfire {
            entry.wildfire = wf;
        }
        if tor > entry.tornado {
            entry.tornado = tor;
        }
        if seis > entry.seismic {
            entry.seismic = seis;
        }
    }
    map
}

