//! Helper `load_t2_stitched_member_proof_intake`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_stitched_member_proof_intake(
    path: &Path,
) -> Result<Vec<T2StitchedMemberProofIntakeRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
