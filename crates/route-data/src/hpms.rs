use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

/// A single HPMS summary record joined to an NHS route segment.
/// PCT_TRUCK is stored as proportion (0.0–1.0).
#[derive(Debug, Clone, Deserialize)]
pub struct HpmsRecord {
    #[serde(rename = "STATE")]
    pub state: String,
    #[serde(rename = "ROUTE_ID")]
    pub route_id: String,
    /// Annual Average Daily Traffic (vehicles/day)
    #[serde(rename = "AADT")]
    pub aadt: Option<u32>,
    /// Truck proportion 0.0–1.0 (aadt_combination / aadt)
    #[serde(rename = "PCT_TRUCK", deserialize_with = "deserialize_pct")]
    pub pct_truck: Option<f32>,
    /// Lane count (both directions combined)
    #[serde(rename = "LANE_COUNT")]
    pub lane_count: Option<u8>,
    /// International Roughness Index (m/km; lower = smoother)
    #[serde(rename = "IRI")]
    pub iri: Option<f32>,
    /// Posted speed limit (mph)
    #[serde(rename = "SPEED_LIMIT", default)]
    pub speed_limit: Option<u8>,
}

/// FHWA Freight Performance Measures — Travel Time reliability by route.
#[derive(Debug, Clone, Deserialize)]
pub struct HpmsFpmRecord {
    #[serde(rename = "ROUTE_ID")]
    pub route_id: String,
    /// Travel Time Index — mean travel time / free-flow travel time
    #[serde(rename = "TTI")]
    pub tti: Option<f32>,
    /// Planning Time Index — 95th-pct travel time / free-flow travel time
    #[serde(rename = "PTI")]
    pub pti: Option<f32>,
}

pub fn read_hpms_csv(path: &Path) -> Result<Vec<HpmsRecord>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize::<HpmsRecord>() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => eprintln!("HPMS parse warning: {e}"),
        }
    }
    Ok(records)
}

pub fn read_hpms_fpm_csv(path: &Path) -> Result<Vec<HpmsFpmRecord>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize::<HpmsFpmRecord>() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => eprintln!("HPMS FPM parse warning: {e}"),
        }
    }
    Ok(records)
}

fn deserialize_pct<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<f32> = Option::deserialize(deserializer)?;
    // Raw HPMS PCT_TRUCK field is 0–100; store as 0.0–1.0
    Ok(opt.map(|v| v / 100.0))
}
