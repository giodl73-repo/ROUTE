//! Helper `load_t3_zone_map_diagnostics`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t3_zone_map_diagnostics(path: &Path) -> Result<Vec<T3ZoneMapDiagnosticRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
