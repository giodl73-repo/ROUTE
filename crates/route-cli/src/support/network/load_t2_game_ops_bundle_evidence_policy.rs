//! Helper `load_t2_game_ops_bundle_evidence_policy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_game_ops_bundle_evidence_policy(
    path: &Path,
) -> Result<Vec<T2GameOpsBundleEvidencePolicyRow>> {
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

