//! Helper `load_tier_pavement_funding_commitment_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_funding_commitment_review(
    path: &Path,
) -> Result<Vec<TierPavementFundingCommitmentReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

