//! Helper `load_tier_pavement_source_access`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_source_access(path: &Path) -> Result<Vec<TierPavementSourceAccessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

