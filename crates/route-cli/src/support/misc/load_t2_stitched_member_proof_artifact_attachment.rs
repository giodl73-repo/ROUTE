//! Helper `load_t2_stitched_member_proof_artifact_attachment`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_stitched_member_proof_artifact_attachment(
    path: &Path,
) -> Result<Vec<T2StitchedMemberProofArtifactAttachmentRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
