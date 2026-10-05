//! Helper `t2_stitched_member_proof_intake_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_intake_rows(
    access_rows: &[T2StitchedMemberSourceAccessPolicyRow],
) -> Vec<T2StitchedMemberProofIntakeRow> {
    let mut rows = access_rows
        .iter()
        .filter(|row| row.evidence_artifact == "source-needed")
        .map(|access| T2StitchedMemberProofIntakeRow {
            proof_intake_id: format!(
                "T2STITCHEDPROOF-{}",
                stable_id_fragment(&access.access_policy_id)
            ),
            access_policy_id: access.access_policy_id.clone(),
            route: access.route.clone(),
            candidate_segment_bundle_id: access.candidate_segment_bundle_id.clone(),
            state_scope: access.state_scope.clone(),
            required_artifact_fields:
                "source title; source url or cached artifact; capture date; route; state scope; source owner"
                    .to_string(),
            required_geometry_statement:
                "route geometry statement explaining continuity with the blocked stitched service"
                    .to_string(),
            proof_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "manual or cached route-geometry proof artifact has not been captured or reviewed"
                    .to_string(),
            blocked_claims_before: access.blocked_claims_after.clone(),
            blocked_claims_after: access.blocked_claims_after.clone(),
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
