//! Helper `load_tier_pavement_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_docket(path: &Path) -> Result<Vec<TierPavementDocketRow>> {
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
