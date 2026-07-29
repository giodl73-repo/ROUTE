//! Helper `load_tier_table_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_table_rows(path: &Path) -> Result<Vec<TierTableScoreRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

