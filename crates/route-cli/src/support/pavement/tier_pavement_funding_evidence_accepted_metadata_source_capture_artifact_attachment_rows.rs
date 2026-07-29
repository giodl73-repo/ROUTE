//! Helper `tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_rows(
    capture_rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow],
) -> Vec<TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow> {
    capture_rows
        .iter()
        .filter(|row| {
            row.source_capture_status == "source-needed"
                && row.captured_artifact == "none"
                && row.captured_source_title == "source-needed"
                && row.captured_source_url == "source-needed"
                && row.captured_commitment_amount_m == "source-needed"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(
            |row| {
                TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow {
                    accepted_metadata_source_capture_artifact_attachment_id: format!(
                        "PAVEMENTFUNDINGACCEPTEDMETASOURCEATTACH-{}",
                        stable_id_fragment(&row.accepted_metadata_source_capture_id)
                    ),
                    accepted_metadata_source_capture_id: row
                        .accepted_metadata_source_capture_id
                        .clone(),
                    evidence_contract_id: row.evidence_contract_id.clone(),
                    state: row.state.clone(),
                    tier: row.tier.clone(),
                    route: row.route.clone(),
                    segment_bundle_id: row.segment_bundle_id.clone(),
                    required_artifact_type: row.required_artifact_type.clone(),
                    attachment_status: "source-needed".to_string(),
                    attached_artifact: "none".to_string(),
                    captured_source_title: "source-needed".to_string(),
                    captured_source_url: "source-needed".to_string(),
                    captured_commitment_amount_m: "source-needed".to_string(),
                    evidence_review_status: "not-reviewed".to_string(),
                    accepted_evidence_status: "not-accepted".to_string(),
                    relief_eligibility: "not-eligible-for-relief".to_string(),
                    blocked_claims_before: row.blocked_claims.clone(),
                    blocked_claims_after: row.blocked_claims.clone(),
                    claim_blocker_delta: 0,
                    attachment_blocker:
                        "accepted full-cost programming or DOT commitment artifact has not been attached"
                            .to_string(),
                    next_action: "review accepted funding artifact only after attachment"
                        .to_string(),
                    next_artifact:
                        "data/tier-pavement-funding-evidence-accepted-metadata-source-capture-artifact-attachment.csv"
                            .to_string(),
                    validation_status: "held".to_string(),
                }
            },
        )
        .collect()
}

