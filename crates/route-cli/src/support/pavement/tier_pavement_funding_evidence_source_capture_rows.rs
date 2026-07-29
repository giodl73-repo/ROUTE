//! Helper `tier_pavement_funding_evidence_source_capture_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_source_capture_rows(
    contract_rows: &[TierPavementFundingEvidenceContractRow],
) -> Vec<TierPavementFundingEvidenceSourceCaptureRow> {
    contract_rows
        .iter()
        .filter(|row| {
            row.accepted_evidence_status == "source-needed"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceSourceCaptureRow {
            source_capture_id: format!(
                "PAVEMENTFUNDINGSOURCE-{}",
                stable_id_fragment(&row.evidence_contract_id)
            ),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            source_capture_status: "source-needed".to_string(),
            captured_artifact: "none".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact for review before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-source-capture.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

