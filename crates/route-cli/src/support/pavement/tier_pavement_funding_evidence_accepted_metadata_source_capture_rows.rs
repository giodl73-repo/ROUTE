//! Helper `tier_pavement_funding_evidence_accepted_metadata_source_capture_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_metadata_source_capture_rows(
    intake_rows: &[TierPavementFundingEvidenceAcceptedMetadataIntakeRow],
) -> Vec<TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow> {
    intake_rows
        .iter()
        .filter(|row| {
            row.intake_status == "artifact-required"
                && row.cache_status == "not-cached"
                && row.evidence_artifact == "source-needed"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(
            |row| TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow {
                accepted_metadata_source_capture_id: format!(
                    "PAVEMENTFUNDINGACCEPTEDMETASOURCE-{}",
                    stable_id_fragment(&row.accepted_metadata_intake_id)
                ),
                accepted_metadata_intake_id: row.accepted_metadata_intake_id.clone(),
                evidence_contract_id: row.evidence_contract_id.clone(),
                state: row.state.clone(),
                tier: row.tier.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                required_artifact_type: row.required_artifact_type.clone(),
                required_source_metadata: row.required_source_metadata.clone(),
                source_capture_status: "source-needed".to_string(),
                captured_artifact: "none".to_string(),
                captured_source_title: "source-needed".to_string(),
                captured_source_url: "source-needed".to_string(),
                captured_commitment_amount_m: "source-needed".to_string(),
                evidence_review_status: "not-reviewed".to_string(),
                accepted_evidence_status: "not-accepted".to_string(),
                relief_eligibility: "not-eligible-for-relief".to_string(),
                blocked_claims: row.blocked_claims.clone(),
                claim_blocker_delta: 0,
                next_action:
                    "attach accepted funding artifact only after source metadata is captured"
                        .to_string(),
                next_artifact:
                    "data/tier-pavement-funding-evidence-accepted-metadata-source-capture.csv"
                        .to_string(),
                validation_status: "held".to_string(),
            },
        )
        .collect()
}

