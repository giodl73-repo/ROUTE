//! Helper `load_tier_pavement_funding_evidence_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_funding_evidence_contract(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceContractRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
