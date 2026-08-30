//! Helper `load_t2_relief_evidence_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_relief_evidence_docket(path: &Path) -> Result<Vec<T2ReliefEvidenceRow>> {
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
