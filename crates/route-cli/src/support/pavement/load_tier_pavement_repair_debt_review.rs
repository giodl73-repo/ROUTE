//! Helper `load_tier_pavement_repair_debt_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_repair_debt_review(
    path: &Path,
) -> Result<Vec<TierPavementRepairDebtReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

