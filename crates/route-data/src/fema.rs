/// FEMA NFHL (National Flood Hazard Layer) data fetch.
///
/// Queries Layer 28 (Flood Hazard Zones / SFHA) of the FEMA ArcGIS REST service:
///   https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query
///
/// fetch_fema_count: single URL → SFHA feature count (for small tiles)
pub fn fetch_fema_count(url: &str) -> anyhow::Result<u32> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .user_agent("ROUTE/1.0 highway-analysis")
        .build()?;
    let text = client
        .get(url)
        .send()
        .map_err(|e| anyhow::anyhow!("FEMA request: {e}"))?
        .text()
        .map_err(|e| anyhow::anyhow!("FEMA body: {e}"))?;
    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| anyhow::anyhow!("FEMA JSON: {e}"))?;
    Ok(json["count"].as_u64().unwrap_or(0) as u32)
}
/// Returns the count of SFHA A-zone features intersecting a bounding box.
/// This is used as a proxy for flood exposure on a corridor (D1 dimension).
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

const FEMA_NFHL_URL: &str =
    "https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query";

/// A T1 corridor bounding box for the FEMA SFHA query.
#[derive(Debug, Clone)]
pub struct CorridorBbox {
    pub corridor: &'static str,
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

/// T1 corridor bounding boxes (approximate degrees).
/// Gulf Coast extent for I-10; northern transcontinental for I-80; etc.
pub const T1_BBOXES: &[CorridorBbox] = &[
    CorridorBbox {
        corridor: "I-10",
        xmin: -94.0,
        ymin: 25.0,
        xmax: -80.0,
        ymax: 31.0,
    },
    CorridorBbox {
        corridor: "I-80",
        xmin: -122.5,
        ymin: 38.0,
        xmax: -74.0,
        ymax: 42.5,
    },
    CorridorBbox {
        corridor: "I-95",
        xmin: -81.0,
        ymin: 25.0,
        xmax: -70.0,
        ymax: 47.5,
    },
    CorridorBbox {
        corridor: "I-35",
        xmin: -97.5,
        ymin: 27.0,
        xmax: -93.0,
        ymax: 46.5,
    },
    CorridorBbox {
        corridor: "I-5",
        xmin: -124.0,
        ymin: 32.5,
        xmax: -117.0,
        ymax: 49.0,
    },
    CorridorBbox {
        corridor: "I-75",
        xmin: -84.5,
        ymin: 25.0,
        xmax: -83.0,
        ymax: 46.0,
    },
    CorridorBbox {
        corridor: "I-90",
        xmin: -122.5,
        ymin: 41.5,
        xmax: -71.0,
        ymax: 48.5,
    },
    CorridorBbox {
        corridor: "I-40",
        xmin: -117.0,
        ymin: 34.0,
        xmax: -74.0,
        ymax: 36.5,
    },
];

/// Result for one corridor SFHA query.
#[derive(Debug, Clone)]
pub struct FemaSfhaResult {
    pub corridor: String,
    pub bbox: String,
    pub sfha_count: u64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
struct FemaCountResponse {
    count: Option<u64>,
    error: Option<FemaError>,
}

#[derive(Debug, Deserialize)]
struct FemaError {
    message: String,
}

/// Query the FEMA NFHL Layer 28 for SFHA A-zone feature count in a bounding box.
pub fn query_sfha_count(client: &reqwest::blocking::Client, bbox: &CorridorBbox) -> FemaSfhaResult {
    let geometry = format!("{},{},{},{}", bbox.xmin, bbox.ymin, bbox.xmax, bbox.ymax);
    let bbox_str = format!("{},{},{},{}", bbox.xmin, bbox.ymin, bbox.xmax, bbox.ymax);

    let resp = client
        .get(FEMA_NFHL_URL)
        .query(&[
            ("where", "FLD_ZONE LIKE 'A%'"),
            ("geometry", &geometry),
            ("geometryType", "esriGeometryEnvelope"),
            ("spatialRel", "esriSpatialRelIntersects"),
            ("returnCountOnly", "true"),
            ("f", "json"),
        ])
        .send();

    match resp {
        Err(e) => FemaSfhaResult {
            corridor: bbox.corridor.to_string(),
            bbox: bbox_str,
            sfha_count: 0,
            status: format!("error: {e}"),
        },
        Ok(r) => {
            let text = r.text().unwrap_or_default();
            match serde_json::from_str::<FemaCountResponse>(&text) {
                Ok(parsed) => {
                    if let Some(err) = parsed.error {
                        FemaSfhaResult {
                            corridor: bbox.corridor.to_string(),
                            bbox: bbox_str,
                            sfha_count: 0,
                            status: format!("api error: {}", err.message),
                        }
                    } else {
                        FemaSfhaResult {
                            corridor: bbox.corridor.to_string(),
                            bbox: bbox_str,
                            sfha_count: parsed.count.unwrap_or(0),
                            status: "ok".to_string(),
                        }
                    }
                }
                Err(e) => FemaSfhaResult {
                    corridor: bbox.corridor.to_string(),
                    bbox: bbox_str,
                    sfha_count: 0,
                    status: format!("parse error: {e}  body={}", &text[..text.len().min(200)]),
                },
            }
        }
    }
}

/// Fetch SFHA counts for all T1 corridors and write to `output_path` as CSV.
/// Columns: corridor, bbox, sfha_count, status
pub fn fetch_all_sfha_counts(output_path: &Path) -> Result<Vec<FemaSfhaResult>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("ROUTE/1.0 highway-analysis")
        .build()
        .context("building HTTP client")?;

    let mut results = Vec::new();
    for bbox in T1_BBOXES {
        print!("  [fema] {}… ", bbox.corridor);
        let r = query_sfha_count(&client, bbox);
        if r.status == "ok" {
            println!("{} SFHA features", r.sfha_count);
        } else {
            println!("FAILED — {}", r.status);
        }
        results.push(r);
    }

    // Write CSV only after all fetch attempts complete, preserving the previous cache on failure.
    let tmp = crate::fetch::temp_path_for(output_path);
    let mut wtr =
        csv::Writer::from_path(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
    wtr.write_record(["corridor", "bbox", "sfha_count", "status"])?;
    for r in &results {
        wtr.write_record(&[
            r.corridor.clone(),
            r.bbox.clone(),
            r.sfha_count.to_string(),
            r.status.clone(),
        ])?;
    }
    wtr.flush()?;
    drop(wtr);
    crate::fetch::replace_with_temp(&tmp, output_path)?;

    Ok(results)
}
