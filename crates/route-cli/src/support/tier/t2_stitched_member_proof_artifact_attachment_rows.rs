//! Helper `t2_stitched_member_proof_artifact_attachment_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_artifact_attachment_rows(
    capture_rows: &[T2StitchedMemberProofSourceCaptureRow],
) -> Vec<T2StitchedMemberProofArtifactAttachmentRow> {
    let mut rows = capture_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|capture| T2StitchedMemberProofArtifactAttachmentRow {
            artifact_attachment_id: format!(
                "T2STITCHEDATTACH-{}",
                stable_id_fragment(&capture.source_capture_id)
            ),
            source_capture_id: capture.source_capture_id.clone(),
            route: capture.route.clone(),
            candidate_segment_bundle_id: capture.candidate_segment_bundle_id.clone(),
            state_scope: capture.state_scope.clone(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            attachment_blocker:
                "manual or cached DOT route-geometry artifact reference has not been attached"
                    .to_string(),
            blocked_claims_before: capture.blocked_claims_after.clone(),
            blocked_claims_after: capture.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
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
