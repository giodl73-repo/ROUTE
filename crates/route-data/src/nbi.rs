use serde::Deserialize;
use std::path::Path;
use anyhow::Result;

/// A bridge record from the FHWA National Bridge Inventory CSV.
/// Joined to corridor by coordinate proximity (R-tree, ≤0.002° with route-name check).
#[derive(Debug, Clone, Deserialize)]
pub struct NbiRecord {
    /// Latitude (decimal degrees, EPSG:4269)
    #[serde(rename = "LAT_016")]
    pub lat: f64,
    /// Longitude (decimal degrees, negative = West)
    #[serde(rename = "LONG_017")]
    pub lon: f64,
    /// Route carried on bridge (used for route-name similarity check)
    #[serde(rename = "FACILITY_CARRIED_007")]
    pub route_on_bridge: String,
    /// Sufficiency rating (0–100; <50 = poor, <25 = critical)
    #[serde(rename = "SUFFICIENCY_RATING")]
    pub sufficiency_rating: Option<f32>,
    /// Year built
    #[serde(rename = "YEAR_BUILT_027")]
    pub year_built: Option<u16>,
    /// Overall condition rating (N=Not applicable, G=Good, F=Fair, P=Poor, C=Critical)
    #[serde(rename = "BRIDGE_CONDITION")]
    pub condition: Option<String>,
}

impl NbiRecord {
    /// True if condition is rated Poor or Critical.
    pub fn is_poor(&self) -> bool {
        matches!(self.condition.as_deref(), Some("P") | Some("C"))
    }
}

pub fn read_nbi_csv(path: &Path) -> Result<Vec<NbiRecord>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize::<NbiRecord>() {
        match result {
            Ok(r) => {
                // Skip records with invalid coordinates
                if r.lat.abs() > 0.001 && r.lon.abs() > 0.001 {
                    records.push(r);
                }
            }
            Err(e) => eprintln!("NBI parse warning: {e}"),
        }
    }
    Ok(records)
}
