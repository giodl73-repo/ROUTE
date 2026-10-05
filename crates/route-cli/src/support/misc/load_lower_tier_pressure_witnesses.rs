//! Helper `load_lower_tier_pressure_witnesses`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_lower_tier_pressure_witnesses(
    path: &Path,
) -> Result<Vec<LowerTierPressureWitnessRow>> {
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
