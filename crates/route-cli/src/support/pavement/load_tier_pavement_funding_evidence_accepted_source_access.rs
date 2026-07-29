//! Helper `load_tier_pavement_funding_evidence_accepted_source_access`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_funding_evidence_accepted_source_access(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedSourceAccessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

