//! Helper `load_tier_candidate_columns`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_candidate_columns(path: &Path) -> Result<Vec<TierCandidateColumnRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

