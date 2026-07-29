//! Helper `load_t2_asset_condition_map_publication_exclusion`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_asset_condition_map_publication_exclusion(
    path: &Path,
) -> Result<Vec<T2AssetConditionMapPublicationExclusionRow>> {
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

