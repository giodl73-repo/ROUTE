//! Native corridor loading; simulation is shared with WebAssembly.
pub use route_kernel::od::*;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CorridorsFile {
    corridor: std::collections::HashMap<String, CorridorRecord>,
}

#[derive(Debug, Deserialize)]
struct CorridorRecord {
    name: String,
    origin: String,
    destination: String,
    fixed_overhead_hours: f64,
    hos_driving_hours: f64,
    hos_rest_hours: f64,
    segments: Vec<SegmentRecord>,
}

#[derive(Debug, Deserialize)]
struct SegmentRecord {
    name: String,
    miles: f64,
    base_vc: f64,
    free_flow_mph: f64,
    incident_prob: f64,
    incident_delay_mean_h: f64,
    incident_delay_std_h: f64,
    managed_lane_bypasses: bool,
    managed_lane_vc: f64,
}

impl From<CorridorRecord> for OdCorridor {
    fn from(r: CorridorRecord) -> Self {
        OdCorridor {
            name: r.name,
            origin: r.origin,
            destination: r.destination,
            fixed_overhead_hours: r.fixed_overhead_hours,
            hos_driving_hours: r.hos_driving_hours,
            hos_rest_hours: r.hos_rest_hours,
            segments: r
                .segments
                .into_iter()
                .map(|s| CorridorSegment {
                    name: s.name,
                    miles: s.miles,
                    base_vc: s.base_vc,
                    free_flow_mph: s.free_flow_mph,
                    incident_prob: s.incident_prob,
                    incident_delay_mean_hours: s.incident_delay_mean_h,
                    incident_delay_std_hours: s.incident_delay_std_h,
                    managed_lane_bypasses_incident: s.managed_lane_bypasses,
                    managed_lane_vc: s.managed_lane_vc,
                })
                .collect(),
        }
    }
}

/// Load a named corridor from data/od-corridors.toml.
/// Falls back to the built-in function if the file is missing or the corridor is not found.
pub fn load_corridor(data_dir: &Path, slug: &str) -> Option<OdCorridor> {
    let path = data_dir.join("od-corridors.toml");
    if path.exists() {
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(file) = toml::from_str::<CorridorsFile>(&text) {
                if let Some(record) = file
                    .corridor
                    .into_iter()
                    .find(|(k, _)| k == slug)
                    .map(|(_, v)| v)
                {
                    return Some(record.into());
                }
            }
        }
    }
    // Fall back to built-in
    match slug {
        "ny_la" => Some(ny_la_corridor()),
        "hou_chi_current" => Some(hou_chi_current()),
        "hou_chi_i69" => Some(hou_chi_i69()),
        "chi_la" => Some(chi_la()),
        "mia_nyc" => Some(mia_nyc()),
        "sea_chi" => Some(sea_chi()),
        "dal_nyc" => Some(dal_nyc()),
        "la_sea" => Some(la_sea()),
        "atl_chi" => Some(atl_chi()),
        "ny_chi" => Some(ny_chi()),
        _ => None,
    }
}
