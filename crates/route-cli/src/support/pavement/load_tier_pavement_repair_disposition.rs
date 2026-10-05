//! Helper `load_tier_pavement_repair_disposition`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_repair_disposition(
    path: &Path,
) -> Result<Vec<TierPavementRepairDispositionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
