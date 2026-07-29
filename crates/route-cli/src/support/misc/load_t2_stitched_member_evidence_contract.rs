//! Helper `load_t2_stitched_member_evidence_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_stitched_member_evidence_contract(
    path: &Path,
) -> Result<Vec<T2StitchedMemberEvidenceContractRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

