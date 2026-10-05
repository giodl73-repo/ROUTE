//! Helper `load_t2_beck_transfer_complexity_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_beck_transfer_complexity_review(
    path: &Path,
) -> Result<Vec<T2BeckTransferComplexityReviewRow>> {
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
