//! Helper `load_tier_pavement_downgrade_exclusion_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_downgrade_exclusion_decision(
    path: &Path,
) -> Result<Vec<TierPavementDowngradeExclusionDecisionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

