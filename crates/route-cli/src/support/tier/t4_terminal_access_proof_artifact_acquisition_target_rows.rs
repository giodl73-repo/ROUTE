//! Helper `t4_terminal_access_proof_artifact_acquisition_target_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_artifact_acquisition_target_rows(
    review_rows: &[T4TerminalAccessProofAttachmentReviewRow],
) -> Vec<T4TerminalAccessProofArtifactAcquisitionTargetRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.source_artifact_reference == "source-needed"
                && row.review_decision == "held-no-source-artifact"
                && row.proof_acceptance_status == "not-accepted"
                && row.validation_status == "review"
        })
        .map(|row| T4TerminalAccessProofArtifactAcquisitionTargetRow {
            acquisition_target_id: format!(
                "T4ACCESSARTIFACTTARGET-{}",
                stable_id_fragment(&row.attachment_review_id)
            ),
            attachment_review_id: row.attachment_review_id.clone(),
            artifact_attachment_id: row.artifact_attachment_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            candidate_source_owner:
                "terminal operator, port authority, state DOT, MPO, or public terminal map"
                    .to_string(),
            required_artifact_fields:
                "source title; source url or cached artifact; capture date; route; terminal; connector; route-to-terminal contact statement"
                    .to_string(),
            prohibited_seed_source: "data/intermodal_terminals.csv".to_string(),
            acquisition_status: "source-needed".to_string(),
            cache_status: "not-cached".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action: "acquire or cache non-seed route-to-terminal proof artifact".to_string(),
            next_artifact:
                "data/t4-terminal-access-proof-artifact-acquisition-targets.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
