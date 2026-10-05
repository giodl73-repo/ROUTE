//! Helper `t4_terminal_access_proof_attachment_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_attachment_review_rows(
    attachment_rows: &[T4TerminalAccessProofArtifactAttachmentRow],
) -> Vec<T4TerminalAccessProofAttachmentReviewRow> {
    let mut rows = attachment_rows
        .iter()
        .filter(|row| {
            row.source_artifact_reference == "source-needed"
                && row.attachment_status == "source-needed"
                && row.evidence_review_status == "not-reviewed"
                && row.proof_acceptance_status == "not-accepted"
                && row.validation_status == "review"
        })
        .map(|row| T4TerminalAccessProofAttachmentReviewRow {
            attachment_review_id: format!(
                "T4ACCESSATTACHREVIEW-{}",
                stable_id_fragment(&row.artifact_attachment_id)
            ),
            artifact_attachment_id: row.artifact_attachment_id.clone(),
            source_capture_id: row.source_capture_id.clone(),
            proof_intake_id: row.proof_intake_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: row.source_artifact_reference.clone(),
            review_decision: "held-no-source-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "proof artifact attachment remains source-needed; terminal-access proof cannot be accepted"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_before.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/optimizer-residual-blocker-backlog.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
