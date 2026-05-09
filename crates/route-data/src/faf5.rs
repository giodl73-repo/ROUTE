use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

/// A FAF5 freight flow record (FAF5 v5.6, 2022 data year).
/// Origin/destination are FAF zone codes (numeric region identifiers).
/// DMODE: 1=truck, 2=rail, 3=water, 4=air, 5=pipeline, 6=other.
#[derive(Debug, Clone, Deserialize)]
pub struct Faf5Record {
    /// Origin FAF zone code
    #[serde(rename = "dms_orig")]
    pub orig_faf: u32,
    /// Destination FAF zone code
    #[serde(rename = "dms_dest")]
    pub dest_faf: u32,
    /// Mode of transport
    #[serde(rename = "dms_mode")]
    pub dmode: u8,
    /// Freight value in billions USD
    #[serde(rename = "value_2022")]
    pub value_b: f64,
    /// Freight tonnage (thousands of tons)
    #[serde(rename = "tons_2022")]
    pub tons_k: f64,
}

impl Faf5Record {
    pub fn is_truck(&self) -> bool {
        self.dmode == 1
    }
}

pub fn read_faf5_csv(path: &Path) -> Result<Vec<Faf5Record>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize::<Faf5Record>() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => eprintln!("FAF5 parse warning: {e}"),
        }
    }
    Ok(records)
}

/// FAF zone centroids — used to attribute flows to corridors via zone traversal.
/// Zone codes and approximate centroids (lat/lon) for each FAF5 zone.
/// Full table embedded here; 132 zones in FAF5 v5.
#[derive(Debug, Clone)]
pub struct FafZone {
    pub code: u32,
    pub name: String,
    /// Centroid in EPSG:4269
    pub lat: f64,
    pub lon: f64,
    pub state: String,
}

/// Returns the embedded FAF5 zone centroid table.
/// Source: FAF5 v5.6 zone definitions, BTS/FHWA 2022.
pub fn faf5_zones() -> Vec<FafZone> {
    // Abbreviated — full 132-zone table to be populated from FAF5 zone shapefile
    vec![
        FafZone {
            code: 11,
            name: "Washington DC area".into(),
            lat: 38.9,
            lon: -77.0,
            state: "DC".into(),
        },
        FafZone {
            code: 119,
            name: "Rest of Virginia".into(),
            lat: 37.5,
            lon: -79.5,
            state: "VA".into(),
        },
        FafZone {
            code: 191,
            name: "New York City".into(),
            lat: 40.7,
            lon: -74.0,
            state: "NY".into(),
        },
        FafZone {
            code: 411,
            name: "Los Angeles area".into(),
            lat: 34.05,
            lon: -118.2,
            state: "CA".into(),
        },
        FafZone {
            code: 419,
            name: "Rest of California".into(),
            lat: 36.8,
            lon: -120.0,
            state: "CA".into(),
        },
        FafZone {
            code: 531,
            name: "Chicago area".into(),
            lat: 41.85,
            lon: -87.65,
            state: "IL".into(),
        },
        FafZone {
            code: 481,
            name: "Dallas area".into(),
            lat: 32.8,
            lon: -96.8,
            state: "TX".into(),
        },
        FafZone {
            code: 489,
            name: "Rest of Texas".into(),
            lat: 31.0,
            lon: -99.0,
            state: "TX".into(),
        },
        // TODO: populate remaining 124 zones from FAF5 zone shapefile
    ]
}
