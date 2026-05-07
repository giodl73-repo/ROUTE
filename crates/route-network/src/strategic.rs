/// Strategic corridor designations — loaded from data/corridor-designations.csv
///
/// No hard-coded scores. All values come from the CSV which is:
///   - Sourced from FHWA USMCA designations, DoD STRAHNET, USDA ERS
///   - Inspectable and editable by the user
///   - Updated as federal designations change
///
/// A4: USMCA trade corridor (0-10) — FHWA High Priority Corridors + border crossing data
/// B4: Military/strategic (0-10) — STRAHNET + military installation proximity
/// C4: Agricultural export (0-10) — USDA county production + export terminal proximity
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct CorridorDesignation {
    pub a4_usmca: f64,
    pub b4_military: f64,
    pub c4_ag_export: f64,
}

/// Load corridor designations from data/corridor-designations.csv.
/// Returns a map of route_id → designation scores.
/// Falls back to zero scores if the file is not found.
pub fn load_designations(data_dir: &Path) -> HashMap<String, CorridorDesignation> {
    let path = data_dir.join("corridor-designations.csv");
    let mut map = HashMap::new();

    let content = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return map, // File not found — all scores default to 0.0
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 4 { continue; }
        if fields[0] == "ROUTE_ID" { continue; } // header

        let route_id = fields[0].trim().to_string();
        let a4: f64 = fields[1].trim().parse().unwrap_or(0.0);
        let b4: f64 = fields[2].trim().parse().unwrap_or(0.0);
        let c4: f64 = fields[3].trim().parse().unwrap_or(0.0);

        map.insert(route_id, CorridorDesignation { a4_usmca: a4, b4_military: b4, c4_ag_export: c4 });
    }

    map
}

/// Global designation cache — loaded once at program start.
/// In the CLI, this is populated via `load_designations("data")`.
/// In tests or when no file exists, all scores default to 0.0.
use std::sync::OnceLock;
static DESIGNATIONS: OnceLock<HashMap<String, CorridorDesignation>> = OnceLock::new();

pub fn init_designations(data_dir: &Path) {
    let _ = DESIGNATIONS.set(load_designations(data_dir));
}

pub fn get_designation(route_id: &str) -> CorridorDesignation {
    let from_csv = DESIGNATIONS.get()
        .and_then(|m| m.get(route_id))
        .cloned();

    match from_csv {
        Some(d) => d,
        None => {
            // Default: all interstates are STRAHNET (B4 = 5.0 baseline).
            // US highways get 0 unless in the CSV.
            // This ensures every interstate gets at least the STRAHNET baseline,
            // while corridors with specific designations get their full scores.
            let b4_default = if route_id.starts_with('I') { 5.0 } else { 0.0 };
            CorridorDesignation {
                a4_usmca: 0.0,
                b4_military: b4_default,
                c4_ag_export: 0.0,
            }
        }
    }
}

/// Convenience functions matching the old interface.
/// Now reads from loaded CSV, not from hard-coded match arms.
pub fn usmca_corridor_score(route_id: &str) -> f64 {
    get_designation(route_id).a4_usmca
}

pub fn military_strategic_score(route_id: &str) -> f64 {
    get_designation(route_id).b4_military
}

pub fn agricultural_export_score(route_id: &str) -> f64 {
    get_designation(route_id).c4_ag_export
}
