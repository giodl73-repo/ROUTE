//! Helper `t2_stitched_member_proof_review_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_review_docket_rows(
    attachment_rows: &[T2StitchedMemberProofArtifactAttachmentRow],
) -> Vec<T2StitchedMemberProofReviewDocketRow> {
    let mut rows = attachment_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|attachment| T2StitchedMemberProofReviewDocketRow {
            proof_review_id: format!(
                "T2STITCHEDREVIEW-{}",
                stable_id_fragment(&attachment.artifact_attachment_id)
            ),
            artifact_attachment_id: attachment.artifact_attachment_id.clone(),
            route: attachment.route.clone(),
            candidate_segment_bundle_id: attachment.candidate_segment_bundle_id.clone(),
            state_scope: attachment.state_scope.clone(),
            source_artifact_reference: attachment.source_artifact_reference.clone(),
            review_decision: "held-no-source-artifact".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            candidate_disposition_status: "not-ready-for-disposition".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "artifact attachment remains source-needed; proof review cannot accept continuity evidence"
                    .to_string(),
            blocked_claims_before: attachment.blocked_claims_after.clone(),
            blocked_claims_after: attachment.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/tier-optimizer-runs.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

