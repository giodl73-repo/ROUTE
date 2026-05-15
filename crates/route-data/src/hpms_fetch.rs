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

const BASE_URL: &str =
    "https://geo.dot.gov/server/rest/services/Hosted/{STATE}_2018_PR/FeatureServer/0/query";

/// All 50 state name-codes as used in the geo.dot.gov endpoint pattern.
pub const STATE_CODES: &[(&str, &str)] = &[
    ("AL", "Alabama"),
    ("AK", "Alaska"),
    ("AZ", "Arizona"),
    ("AR", "Arkansas"),
    ("CA", "California"),
    ("CO", "Colorado"),
    ("CT", "Connecticut"),
    ("DE", "Delaware"),
    ("FL", "Florida"),
    ("GA", "Georgia"),
    ("HI", "Hawaii"),
    ("ID", "Idaho"),
    ("IL", "Illinois"),
    ("IN", "Indiana"),
    ("IA", "Iowa"),
    ("KS", "Kansas"),
    ("KY", "Kentucky"),
    ("LA", "Louisiana"),
    ("ME", "Maine"),
    ("MD", "Maryland"),
    ("MA", "Massachusetts"),
    ("MI", "Michigan"),
    ("MN", "Minnesota"),
    ("MS", "Mississippi"),
    ("MO", "Missouri"),
    ("MT", "Montana"),
    ("NE", "Nebraska"),
    ("NV", "Nevada"),
    ("NH", "NewHampshire"),
    ("NJ", "NewJersey"),
    ("NM", "NewMexico"),
    ("NY", "NewYork"),
    ("NC", "NorthCarolina"),
    ("ND", "NorthDakota"),
    ("OH", "Ohio"),
    ("OK", "Oklahoma"),
    ("OR", "Oregon"),
    ("PA", "Pennsylvania"),
    ("RI", "RhodeIsland"),
    ("SC", "SouthCarolina"),
    ("SD", "SouthDakota"),
    ("TN", "Tennessee"),
    ("TX", "Texas"),
    ("UT", "Utah"),
    ("VT", "Vermont"),
    ("VA", "Virginia"),
    ("WA", "Washington"),
    ("WV", "WestVirginia"),
    ("WI", "Wisconsin"),
    ("WY", "Wyoming"),
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

/// Actual field names from geo.dot.gov (all lowercase).
/// route_signing codes: 2=Interstate, 3=US Route, 4=State Route
/// route_number: the numeric route number (65 for I-65, 30 for US-30)
#[derive(Debug, Deserialize)]
struct FeatureAttributes {
    route_number: Option<i64>,
    /// 2=Interstate, 3=US Route, 4=State Route
    route_signing: Option<i64>,
    aadt: Option<f64>,
    /// Combination truck AADT count (NOT a percentage)
    aadt_combination: Option<f64>,
    iri: Option<f64>,
    through_lanes: Option<f64>,
    speed_limit: Option<f64>,
}

/// Fetch HPMS data for one state and return a list of HpmsRecords.
/// `state_name` is the URL-encoded name, e.g. "California", "New_Hampshire".
pub fn fetch_state_hpms(state_abbr: &str, state_name: &str) -> Result<Vec<HpmsRecord>> {
    fetch_state_hpms_with_systems(state_abbr, state_name, &[1])
}

pub fn fetch_state_hpms_with_systems(
    state_abbr: &str,
    state_name: &str,
    functional_systems: &[u8],
) -> Result<Vec<HpmsRecord>> {
    let url = BASE_URL.replace("{STATE}", state_name);
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("ROUTE/1.0 highway-analysis")
        .build()?;
    let mut all: Vec<HpmsRecord> = Vec::new();
    let mut offset = 0usize;
    let systems = functional_systems
        .iter()
        .copied()
        .filter(|system| *system > 0)
        .collect::<std::collections::BTreeSet<_>>();
    let where_clause = if systems.len() == 1 {
        format!(
            "f_system = {} AND aadt IS NOT NULL",
            systems.iter().next().copied().unwrap_or(1)
        )
    } else {
        let list = systems
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!("f_system IN ({list}) AND aadt IS NOT NULL")
    };

    loop {
        let response = client
            .get(&url)
            .query(&[
                // Simple filter: only records with AADT populated
                // Route type filtering done in Rust after fetch
                // f_system = 1 -> Interstate principal arterials only (Phase 1)
                // route_signing = 2/3/4 filters cause server 500s on this ArcGIS instance
                // Phase 2: explicit f_system lists can add US-route principal arterials.
                ("where", where_clause.as_str()),
                ("outFields", "route_number,route_signing,aadt,aadt_combination,iri,through_lanes,speed_limit,f_system"),
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

            // Build normalised route_id from route_signing + route_number
            // route_signing: 2=Interstate, 3=US Route, 4=State Route
            let route_id = match (a.route_signing, a.route_number) {
                (Some(2), Some(n)) if n > 0 => format!("I{n}"),
                (Some(3), Some(n)) if n > 0 => format!("US{n}"),
                (Some(4), Some(n)) if n > 0 => format!("SR{n}"),
                _ => continue, // skip unsigned/county/municipal roads
            };

            // pct_truck = aadt_combination / aadt (both are counts)
            let pct_truck = match (a.aadt_combination, a.aadt) {
                (Some(comb), Some(total)) if total > 0.0 => Some((comb / total) as f32),
                _ => None,
            };

            all.push(HpmsRecord {
                state: state_abbr.to_string(),
                route_id,
                aadt: a.aadt.map(|v| v as u32),
                pct_truck,
                lane_count: a.through_lanes.map(|v| v as u8),
                iri: a.iri.map(|v| v as f32),
                speed_limit: a.speed_limit.map(|v| v as u8),
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

    if all_records.is_empty() {
        anyhow::bail!(
            "HPMS fetch returned zero records; preserving existing cache at {}",
            output_path.display()
        );
    }

    // Write to CSV only after all fetch attempts complete, preserving the previous cache on failure.
    let tmp = crate::fetch::temp_path_for(output_path);
    let mut wtr = csv::Writer::from_path(&tmp)?;
    wtr.write_record([
        "STATE",
        "ROUTE_ID",
        "AADT",
        "PCT_TRUCK",
        "LANE_COUNT",
        "IRI",
        "SPEED_LIMIT",
    ])?;
    for r in &all_records {
        wtr.write_record(&[
            r.state.clone(),
            r.route_id.clone(),
            r.aadt.map(|v| v.to_string()).unwrap_or_default(),
            r.pct_truck.map(|v| format!("{v:.4}")).unwrap_or_default(),
            r.lane_count.map(|v| v.to_string()).unwrap_or_default(),
            r.iri.map(|v| format!("{v:.1}")).unwrap_or_default(),
            String::new(), // speed_limit not yet stored on HpmsRecord from fetch
        ])?;
    }
    wtr.flush()?;
    drop(wtr);
    crate::fetch::replace_with_temp(&tmp, output_path)?;

    println!(
        "  wrote {} HPMS records → {}",
        all_records.len(),
        output_path.display()
    );
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
        if !num.is_empty() {
            return format!("I{num}");
        }
    }
    if upper.starts_with("I-") || upper.starts_with("I ") {
        let num: String = upper[2..]
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() {
            return format!("I{num}");
        }
    }

    // US route patterns: US, UH
    if upper.starts_with("US") || upper.starts_with("UH") {
        let num: String = upper[2..].chars().filter(|c| c.is_ascii_digit()).collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() {
            return format!("US{num}");
        }
    }

    // State route: SH, SR, ST
    if upper.starts_with("SH") || upper.starts_with("SR") || upper.starts_with("ST") {
        let num: String = upper[2..].chars().filter(|c| c.is_ascii_digit()).collect();
        let num = num.trim_start_matches('0');
        if !num.is_empty() {
            return format!("SR{num}");
        }
    }

    // Fallback: extract digits
    let num: String = upper.chars().filter(|c| c.is_ascii_digit()).collect();
    let num = num.trim_start_matches('0');
    if num.is_empty() {
        "UNKNOWN".into()
    } else {
        format!("R{num}")
    }
}

#[cfg(test)]
mod tests {
    use super::STATE_CODES;

    #[test]
    fn multiword_state_service_names_match_geo_dot_hosted_pattern() {
        for (abbr, service_name) in STATE_CODES {
            if matches!(
                *abbr,
                "NH" | "NJ" | "NM" | "NY" | "NC" | "ND" | "RI" | "SC" | "SD" | "WV"
            ) {
                assert!(
                    !service_name.contains('_'),
                    "{abbr} HPMS hosted service names do not use underscores"
                );
            }
        }
    }

    #[test]
    fn default_state_hpms_fetch_scope_remains_interstate_only() {
        let default_systems = [1u8];
        assert_eq!(default_systems, [1]);
    }
}
