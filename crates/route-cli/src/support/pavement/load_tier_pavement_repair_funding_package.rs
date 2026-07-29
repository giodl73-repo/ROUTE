//! Helper `load_tier_pavement_repair_funding_package`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_repair_funding_package(
    path: &Path,
) -> Result<Vec<TierPavementRepairFundingPackageRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

