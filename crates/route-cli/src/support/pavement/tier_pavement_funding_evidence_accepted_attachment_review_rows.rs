//! Helper `tier_pavement_funding_evidence_accepted_attachment_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_attachment_review_rows(
    attachment_rows: &[TierPavementFundingEvidenceAcceptedArtifactAttachmentRow],
) -> Vec<TierPavementFundingEvidenceAcceptedAttachmentReviewRow> {
    attachment_rows
        .iter()
        .filter(|row| {
            row.attachment_status == "source-needed"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcceptedAttachmentReviewRow {
            accepted_attachment_review_id: format!(
                "PAVEMENTFUNDINGACCEPTEDREVIEW-{}",
                stable_id_fragment(&row.accepted_artifact_attachment_id)
            ),
            accepted_artifact_attachment_id: row.accepted_artifact_attachment_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: row.required_artifact_type.clone(),
            attached_artifact: row.attached_artifact.clone(),
            review_decision: "held-no-attached-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            review_reason:
                "accepted artifact attachment remains source-needed; funding evidence cannot be reviewed or accepted"
                    .to_string(),
            blocked_claims_before: row.blocked_claims_after.clone(),
            blocked_claims_after: row.blocked_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact before evidence review or relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-attachment-review.csv"
                .to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
