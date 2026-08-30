//! Helper `tier_pavement_funding_evidence_artifact_attachment_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_artifact_attachment_rows(
    capture_rows: &[TierPavementFundingEvidenceSourceCaptureRow],
) -> Vec<TierPavementFundingEvidenceArtifactAttachmentRow> {
    capture_rows
        .iter()
        .filter(|row| {
            row.source_capture_status == "source-needed"
                && row.captured_artifact == "none"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceArtifactAttachmentRow {
            artifact_attachment_id: format!(
                "PAVEMENTFUNDINGATTACH-{}",
                stable_id_fragment(&row.source_capture_id)
            ),
            source_capture_id: row.source_capture_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            attachment_status: "source-needed".to_string(),
            attached_artifact: "none".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims_before: row.blocked_claims.clone(),
            blocked_claims_after: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            attachment_blocker:
                "accepted full-cost programming or DOT commitment artifact has not been attached"
                    .to_string(),
            next_action: "attach accepted funding artifact for review before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-artifact-attachment.csv"
                .to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
