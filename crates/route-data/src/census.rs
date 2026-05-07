/// Census Bureau data: county centroids (Gazetteer) + population (ACS).
use serde::Deserialize;
use std::path::Path;
use anyhow::{Context, Result};

/// A US county with its internal point centroid and basic attributes.
#[derive(Debug, Clone)]
pub struct CountyCentroid {
    /// 2-letter state abbreviation
    pub state: String,
    /// 5-digit FIPS code (SS + CCC)
    pub geoid: String,
    /// County name
    pub name: String,
    /// Land area in square miles
    pub aland_sqmi: f64,
    /// Internal point latitude (Census-computed, guaranteed to be on land)
    pub lat: f64,
    /// Internal point longitude
    pub lon: f64,
    /// ACS population (joined separately; 0 if not yet joined)
    pub population: u64,
    /// USDA rural-urban continuum code (1–9; ≥4 = rural; 0 = not assigned)
    pub rucc: u8,
}

/// Parse the Census 2023 Gazetteer county file (tab-separated, extracted from zip).
/// Expected columns: USPS, GEOID, ANSICODE, NAME, ALAND, AWATER, ALAND_SQMI, AWATER_SQMI, INTPTLAT, INTPTLONG
pub fn read_county_gazetteer(path: &Path) -> Result<Vec<CountyCentroid>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading gazetteer {}", path.display()))?;

    let mut counties = Vec::new();
    let mut header_skipped = false;

    for line in content.lines() {
        if !header_skipped {
            header_skipped = true;
            continue;
        }
        let line = line.trim();
        if line.is_empty() { continue; }

        // Tab-separated: USPS GEOID ANSICODE NAME ALAND AWATER ALAND_SQMI AWATER_SQMI INTPTLAT INTPTLONG
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 10 { continue; }

        let state = fields[0].trim().to_string();
        let geoid = fields[1].trim().to_string();
        let name = fields[3].trim().to_string();
        let aland_sqmi: f64 = fields[6].trim().parse().unwrap_or(0.0);
        let lat: f64 = fields[8].trim().parse().unwrap_or(0.0);
        let lon: f64 = fields[9].trim().parse().unwrap_or(0.0);

        // Skip if coordinates are missing or clearly invalid
        if lat == 0.0 || lon == 0.0 || lat < 24.0 || lat > 72.0 {
            continue;
        }
        // Skip territories (PR, VI, GU, MP, AS) — focus on 50 states + DC
        if matches!(state.as_str(), "PR" | "VI" | "GU" | "MP" | "AS") {
            continue;
        }

        counties.push(CountyCentroid {
            state, geoid, name, aland_sqmi, lat, lon,
            population: 0,
            rucc: 0,
        });
    }

    Ok(counties)
}

/// Fetch ACS 5-year county population from Census API (no auth required).
/// Writes to `output_path` as CSV: GEOID, NAME, POPULATION
pub fn fetch_acs_population(output_path: &Path) -> Result<()> {
    let url = "https://api.census.gov/data/2022/acs/acs5\
               ?get=NAME,B01003_001E&for=county:*&in=state:*";

    println!("  fetching ACS county population from Census API…");
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .user_agent("ROUTE/1.0 highway-analysis")
        .build()?;

    let text = client.get(url).send()
        .context("fetching ACS")?
        .text()
        .context("reading ACS response")?;

    // Response is a JSON array: [["NAME","B01003_001E","state","county"], [...], ...]
    let rows: Vec<Vec<serde_json::Value>> = serde_json::from_str(&text)
        .context("parsing ACS JSON")?;

    let mut wtr = csv::Writer::from_path(output_path)
        .with_context(|| format!("creating {}", output_path.display()))?;
    wtr.write_record(["GEOID", "NAME", "POPULATION"])?;

    for row in rows.iter().skip(1) { // skip header row
        if row.len() < 4 { continue; }
        let name = row[0].as_str().unwrap_or("").to_string();
        let pop: u64 = row[1].as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let state_fips = row[2].as_str().unwrap_or("");
        let county_fips = row[3].as_str().unwrap_or("");
        let geoid = format!("{state_fips}{county_fips}");
        wtr.write_record(&[geoid, name, pop.to_string()])?;
    }
    wtr.flush()?;
    println!("  wrote {} county population records", rows.len().saturating_sub(1));
    Ok(())
}

/// Join ACS population data onto county centroids by GEOID.
pub fn join_population(counties: &mut Vec<CountyCentroid>, pop_csv: &Path) -> Result<usize> {
    use std::collections::HashMap;

    let mut rdr = csv::Reader::from_path(pop_csv)?;
    let mut pop_map: HashMap<String, u64> = HashMap::new();

    for result in rdr.records() {
        let rec = result?;
        if rec.len() >= 3 {
            let geoid = rec[0].to_string();
            let pop: u64 = rec[2].parse().unwrap_or(0);
            pop_map.insert(geoid, pop);
        }
    }

    let mut joined = 0;
    for county in counties.iter_mut() {
        if let Some(&pop) = pop_map.get(&county.geoid) {
            county.population = pop;
            joined += 1;
        }
    }
    Ok(joined)
}
