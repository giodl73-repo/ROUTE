//! Helper `load_tier_pavement_repair_funding_acceptance`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_repair_funding_acceptance(
    path: &Path,
) -> Result<Vec<TierPavementRepairFundingAcceptanceRow>> {
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

