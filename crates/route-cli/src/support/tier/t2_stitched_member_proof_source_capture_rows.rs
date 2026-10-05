//! Helper `t2_stitched_member_proof_source_capture_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_source_capture_rows(
    intake_rows: &[T2StitchedMemberProofIntakeRow],
) -> Vec<T2StitchedMemberProofSourceCaptureRow> {
    let mut rows = intake_rows
        .iter()
        .filter(|row| row.proof_artifact == "source-needed")
        .map(|intake| T2StitchedMemberProofSourceCaptureRow {
            source_capture_id: format!(
                "T2STITCHEDSOURCE-{}",
                stable_id_fragment(&intake.proof_intake_id)
            ),
            proof_intake_id: intake.proof_intake_id.clone(),
            route: intake.route.clone(),
            candidate_segment_bundle_id: intake.candidate_segment_bundle_id.clone(),
            state_scope: intake.state_scope.clone(),
            source_artifact_reference: "source-needed".to_string(),
            source_artifact_type: "manual-or-cached-route-geometry".to_string(),
            capture_status: "source-needed".to_string(),
            evidence_acceptance_status: "not-reviewed".to_string(),
            capture_blocker:
                "manual or cached DOT route-geometry source artifact has not been attached"
                    .to_string(),
            blocked_claims_before: intake.blocked_claims_after.clone(),
            blocked_claims_after: intake.blocked_claims_after.clone(),
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
