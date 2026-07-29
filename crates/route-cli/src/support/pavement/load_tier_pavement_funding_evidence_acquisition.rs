//! Helper `load_tier_pavement_funding_evidence_acquisition`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_funding_evidence_acquisition(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcquisitionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

