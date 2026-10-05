//! Helper `load_tier_pavement_source_fetch_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_source_fetch_review(
    path: &Path,
) -> Result<Vec<TierPavementSourceFetchReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
