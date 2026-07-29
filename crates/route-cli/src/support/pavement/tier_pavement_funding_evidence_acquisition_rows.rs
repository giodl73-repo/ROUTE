//! Helper `tier_pavement_funding_evidence_acquisition_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_acquisition_rows(
    review_rows: &[TierPavementFundingEvidenceReviewDocketRow],
) -> Vec<TierPavementFundingEvidenceAcquisitionRow> {
    review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "held-no-attached-artifact"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcquisitionRow {
            funding_evidence_acquisition_id: format!(
                "PAVEMENTFUNDINGACQUIRE-{}",
                stable_id_fragment(&row.funding_evidence_review_id)
            ),
            funding_evidence_review_id: row.funding_evidence_review_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            acquisition_status: "source-needed".to_string(),
            candidate_source_owner: format!("{} DOT or accepted programming authority", row.state),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims_after.clone(),
            claim_blocker_delta: 0,
            acquisition_reason:
                "funding evidence review is held because no accepted artifact is attached"
                    .to_string(),
            next_action: "acquire accepted full-cost funding artifact before attachment and review"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-acquisition.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

