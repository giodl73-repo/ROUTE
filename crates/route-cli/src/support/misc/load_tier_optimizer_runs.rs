//! Helper `load_tier_optimizer_runs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_optimizer_runs(path: &Path) -> Result<Vec<TierOptimizerRunRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

