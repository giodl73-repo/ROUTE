//! Helper `load_tier_pavement_funding_evidence_accepted_artifact_acquisition`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_funding_evidence_accepted_artifact_acquisition(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
