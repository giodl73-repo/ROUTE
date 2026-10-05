//! Helper `t4_terminal_access_proof_artifact_attachment_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_artifact_attachment_rows(
    capture_rows: &[T4TerminalAccessProofSourceCaptureRow],
) -> Vec<T4TerminalAccessProofArtifactAttachmentRow> {
    let mut rows = capture_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| T4TerminalAccessProofArtifactAttachmentRow {
            artifact_attachment_id: format!(
                "T4ACCESSATTACH-{}",
                stable_id_fragment(&row.source_capture_id)
            ),
            source_capture_id: row.source_capture_id.clone(),
            proof_intake_id: row.proof_intake_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            attachment_blocker:
                "manual or cached non-seed terminal-access proof artifact has not been attached"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-attachment-review.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
