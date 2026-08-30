//! Helper `load_t3_lower_tier_feeder_gap_policy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t3_lower_tier_feeder_gap_policy(
    path: &Path,
) -> Result<Vec<T3LowerTierFeederGapPolicyRow>> {
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
