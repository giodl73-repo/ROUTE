/// Fetch HPMS 2018 data from FHWA's ArcGIS REST endpoints at geo.dot.gov.
/// No registration required. Returns per-route aggregated traffic attributes.
///
/// Endpoint pattern:
///   https://geo.dot.gov/server/rest/services/Hosted/{State}_2018_PR/FeatureServer/0/query
///   ?where=1=1&outFields=ROUTE_ID,AADT,IRI,THROUGH_LANES,PCT_COMBINATION&f=json
///   &resultOffset=0&resultRecordCount=1000
use crate::hpms::HpmsRecord;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

const BASE_URL: &str =
    "https://geo.dot.gov/server/rest/services/Hosted/{STATE}_2018_PR/FeatureServer/0/query";

/// All 50 state name-codes as used in the geo.dot.gov endpoint pattern.
pub const STATE_CODES: &[(&str, &str)] = &[
    ("AL", "Alabama"), ("AK", "Alaska"), ("AZ", "Arizona"), ("AR", "Arkansas"),
    ("CA", "California"), ("CO", "Colorado"), ("CT", "Connecticut"), ("DE", "Delaware"),
    ("FL", "Florida"), ("GA", "Georgia"), ("HI", "Hawaii"), ("ID", "Idaho"),
    ("IL", "Illinois"), ("IN", "Indiana"), ("IA", "Iowa"), ("KS", "Kansas"),
    ("KY", "Kentucky"), ("LA", "Louisiana"), ("ME", "Maine"), ("MD", "Maryland"),
    ("MA", "Massachusetts"), ("MI", "Michigan"), ("MN", "Minnesota"), ("MS", "Mississippi"),
    ("MO", "Missouri"), ("MT", "Montana"), ("NE", "Nebraska"), ("NV", "Nevada"),
    ("NH", "New_Hampshire"), ("NJ", "New_Jersey"), ("NM", "New_Mexico"), ("NY", "New_York"),
    ("NC", "North_Carolina"), ("ND", "North_Dakota"), ("OH", "Ohio"), ("OK", "Oklahoma"),
    ("OR", "Oregon"), ("PA", "Pennsylvania"), ("RI", "Rhode_Island"), ("SC", "South_Carolina"),
    ("SD", "South_Dakota"), ("TN", "Tennessee"), ("TX", "Texas"), ("UT", "Utah"),
    ("VT", "Vermont"), ("VA", "Virginia"), ("WA", "Washington"), ("WV", "West_Virginia"),
    ("WI", "Wisconsin"), ("WY", "Wyoming"),
];

#[derive(Debug, Deserialize)]
struct FeatureResponse {
    features: Vec<Feature>,
    #[serde(rename = "exceededTransferLimit")]
    exceeded_transfer_limit: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Feature {
    attributes: FeatureAttributes,
}

#[derive(Debug, Deserialize)]
struct FeatureAttributes {
    #[serde(rename = "ROUTE_ID", alias = "Route_ID")]
    route_id: Option<String>,
    #[serde(rename = "AADT", alias = "Aadt")]
    aadt: Option<f64>,
    #[serde(rename = "IRI", alias = "Iri")]
    iri: Option<f64>,
    #[serde(rename = "THROUGH_LANES", alias = "Through_Lanes")]
    through_lanes: Option<f64>,
    /// PCT_COMBINATION = heavy truck percentage (0–100 in HPMS)
    #[serde(rename = "PCT_COMBINATION", alias = "Pct_Combination")]
    pct_combination: Option<f64>,
}

/// Fetch HPMS data for one state and return a list of HpmsRecords.
/// `state_name` is the URL-encoded name, e.g. "California", "New_Hampshire".
pub fn fetch_state_hpms(
    state_abbr: &str,
    state_name: &str,
) -> Result<Vec<HpmsRecord>> {
    let url = BASE_URL.replace("{STATE}", state_name);
    let client = reqwest::blocking::Client::new();
    let mut all: Vec<HpmsRecord> = Vec::new();
    let mut offset = 0usize;

    loop {
        let response = client
            .get(&url)
            .query(&[
                ("where", "1=1"),
                ("outFields", "ROUTE_ID,AADT,IRI,THROUGH_LANES,PCT_COMBINATION"),
                ("f", "json"),
                ("resultRecordCount", "1000"),
                ("resultOffset", &offset.to_string()),
            ])
            .send()
            .with_context(|| format!("fetching HPMS for {state_abbr}"))?;

        if !response.status().is_success() {
            eprintln!("  [warn] HPMS {state_abbr}: HTTP {}", response.status());
            break;
        }

        let text = response.text()?;
        let parsed: FeatureResponse = serde_json::from_str(&text)
            .with_context(|| format!("parsing HPMS JSON for {state_abbr}"))?;

        let count = parsed.features.len();
        for feat in parsed.features {
            let a = feat.attributes;
            let route_id = match a.route_id {
                Some(r) if !r.trim().is_empty() => normalise_hpms_route_id(&r),
                _ => continue,
            };
            all.push(HpmsRecord {
                state: state_abbr.to_string(),
                route_id,
                aadt: a.aadt.map(|v| v as u32),
                // HPMS PCT_COMBINATION is 0–100; store as proportion 0.0–1.0
                pct_truck: a.pct_combination.map(|v| (v / 100.0) as f32),
                lane_count: a.through_lanes.map(|v| v as u8),
                iri: a.iri.map(|v| v as f32),
            });
        }

        if count < 1000 || parsed.exceeded_transfer_limit != Some(true) {
            break; // last page
        }
        offset += count;
    }

    Ok(all)
}

/// Fetch HPMS for all 50 states, aggregate to route level, save to CSV.
pub fn fetch_all_hpms(output_path: &std::path::Path) -> Result<()> {
    let mut all_records: Vec<HpmsRecord> = Vec::new();

    for (abbr, name) in STATE_CODES {
        print!("  [hpms] {abbr}… ");
        match fetch_state_hpms(abbr, name) {
            Ok(records) => {
                println!("{} segments", records.len());
                all_records.extend(records);
            }
            Err(e) => {
                println!("FAILED — {e}");
                eprintln!("  [warn] skipping {abbr}: {e}");
            }
        }
    }

    // Write to CSV
    let mut wtr = csv::Writer::from_path(output_path)?;
    wtr.write_record(["STATE", "ROUTE_ID", "AADT", "PCT_TRUCK", "LANE_COUNT", "IRI"])?;
    for r in &all_records {
        wtr.write_record(&[
            r.state.clone(),
            r.route_id.clone(),
            r.aadt.map(|v| v.to_string()).unwrap_or_default(),
            r.pct_truck.map(|v| format!("{v:.4}")).unwrap_or_default(),
            r.lane_count.map(|v| v.to_string()).unwrap_or_default(),
            r.iri.map(|v| format!("{v:.1}")).unwrap_or_default(),
        ])?;
    }
    wtr.flush()?;

    println!("  wrote {} HPMS records → {}", all_records.len(), output_path.display());
    Ok(())
}

/// Normalise HPMS Route_ID to our internal format.
/// HPMS route IDs are state-specific but usually contain the route number.
/// "IH0080" → "I80", "US0030" → "US30", "SH0097" → "SR97"
pub fn normalise_hpms_route_id(raw: &str) -> String {
    let upper = raw.trim().to_uppercase();

    // Interstate patterns: IH, IS, I-
    if upper.starts_with("IH") || upper.starts_with("IS") {
        let num: String = upper.chars().filter(|c| c.is_ascii_digit()).collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() { return format!("I{num}"); }
    }
    if upper.starts_with("I-") || upper.starts_with("I ") {
        let num: String = upper[2..].chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() { return format!("I{num}"); }
    }

    // US route patterns: US, UH
    if upper.starts_with("US") || upper.starts_with("UH") {
        let num: String = upper[2..].chars().filter(|c| c.is_ascii_digit()).collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() { return format!("US{num}"); }
    }

    // State route: SH, SR, ST
    if upper.starts_with("SH") || upper.starts_with("SR") || upper.starts_with("ST") {
        let num: String = upper[2..].chars().filter(|c| c.is_ascii_digit()).collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() { return format!("SR{num}"); }
    }

    // Fallback: extract digits
    let num: String = upper.chars().filter(|c| c.is_ascii_digit()).collect();
    let num = num.trim_start_matches('0');
    if num.is_empty() { "UNKNOWN".into() } else { format!("R{num}") }
}
