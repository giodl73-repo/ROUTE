//! Helper `load_optimizer_claim_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_optimizer_claim_review(path: &Path) -> Result<Vec<OptimizerClaimReviewRow>> {
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
