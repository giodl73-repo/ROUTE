//! Helper `load_tier_segment_candidates`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_segment_candidates(path: &Path) -> Result<Vec<TierSegmentCandidateRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

