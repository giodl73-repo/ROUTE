//! Helper `load_tier_region_repairs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_region_repairs(path: &Path) -> Result<Vec<TierRegionRepairInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
