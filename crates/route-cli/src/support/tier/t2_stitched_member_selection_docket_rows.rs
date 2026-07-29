//! Helper `t2_stitched_member_selection_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_selection_docket_rows(
    split_rows: &[T2StitchedMemberSplitPlanRow],
) -> Vec<T2StitchedMemberSelectionDocketRow> {
    let mut rows = split_rows
        .iter()
        .map(|split| T2StitchedMemberSelectionDocketRow {
            selection_docket_id: format!(
                "T2STITCHEDSELECT-{}",
                stable_id_fragment(&split.split_plan_id)
            ),
            split_plan_id: split.split_plan_id.clone(),
            route: split.route.clone(),
            blocked_segment_bundle_id: split.blocked_segment_bundle_id.clone(),
            candidate_segment_bundle_id: split.candidate_segment_bundle_id.clone(),
            state_scope: split.state_scope.clone(),
            candidate_member_count: split.candidate_member_count,
            candidate_length_miles: split.candidate_length_miles,
            selection_decision: "evidence-needed".to_string(),
            selection_action: "collect-state-scope-evidence-before-decision".to_string(),
            evidence_requirement:
                "manual route-family service continuity evidence before in-scope or rejected status"
                    .to_string(),
            blocked_claims_before: split.blocked_claims_after.clone(),
            blocked_claims_after: split.blocked_claims_after.clone(),
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

