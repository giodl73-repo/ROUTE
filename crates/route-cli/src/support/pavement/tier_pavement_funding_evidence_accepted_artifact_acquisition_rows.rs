//! Helper `tier_pavement_funding_evidence_accepted_artifact_acquisition_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_artifact_acquisition_rows(
    review_rows: &[TierPavementFundingEvidenceAcceptedAttachmentReviewRow],
) -> Vec<TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow> {
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
        .map(
            |row| TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow {
                accepted_artifact_acquisition_id: format!(
                    "PAVEMENTFUNDINGACCEPTEDACQUIRE-{}",
                    stable_id_fragment(&row.accepted_attachment_review_id)
                ),
                accepted_attachment_review_id: row.accepted_attachment_review_id.clone(),
                evidence_contract_id: row.evidence_contract_id.clone(),
                state: row.state.clone(),
                tier: row.tier.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                required_artifact_type: row.required_artifact_type.clone(),
                acquisition_status: "source-needed".to_string(),
                cache_status: "not-cached".to_string(),
                candidate_source_owner: format!(
                    "{} DOT or accepted programming authority",
                    row.state
                ),
                accepted_evidence_status: "not-accepted".to_string(),
                relief_eligibility: "not-eligible-for-relief".to_string(),
                blocked_claims: row.blocked_claims_after.clone(),
                claim_blocker_delta: 0,
                acquisition_reason:
                    "accepted funding artifact is not attached and cannot be reviewed".to_string(),
                next_action: "acquire or cache accepted full-cost funding artifact".to_string(),
                next_artifact:
                    "data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv"
                        .to_string(),
                validation_status: "held".to_string(),
            },
        )
        .collect()
}

