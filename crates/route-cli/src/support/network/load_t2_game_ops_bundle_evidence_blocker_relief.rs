//! Helper `load_t2_game_ops_bundle_evidence_blocker_relief`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_game_ops_bundle_evidence_blocker_relief(
    path: &Path,
) -> Result<Vec<T2GameOpsBundleEvidenceBlockerReliefRow>> {
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
