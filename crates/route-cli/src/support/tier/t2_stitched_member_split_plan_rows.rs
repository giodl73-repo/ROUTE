//! Helper `t2_stitched_member_split_plan_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_split_plan_rows(
    decision_rows: &[T2StitchedMemberDecisionDocketRow],
    candidate_rows: &[TierSegmentCandidateRow],
) -> Vec<T2StitchedMemberSplitPlanRow> {
    let mut rows = Vec::new();
    for decision in decision_rows
        .iter()
        .filter(|row| row.decision == "split" && row.validation_status == "review")
    {
        let route_key = canonical_route_key(&decision.route);
        let mut by_bundle =
            std::collections::BTreeMap::<String, Vec<&TierSegmentCandidateRow>>::new();
        for candidate in candidate_rows.iter().filter(|row| {
            canonical_route_key(&row.route) == route_key && row.member_role == "stitched-member"
        }) {
            by_bundle
                .entry(candidate.segment_bundle_id.clone())
                .or_default()
                .push(candidate);
        }

        for (candidate_bundle_id, members) in by_bundle {
            let state_scope = members
                .iter()
                .filter_map(|row| {
                    let state = row.state.trim();
                    if state.is_empty() {
                        None
                    } else {
                        Some(state.to_string())
                    }
                })
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(";");
            let stitch_group_ids = members
                .iter()
                .map(|row| row.stitch_group_id.clone())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(";");
            let length_miles =
                (members.iter().map(|row| row.length_miles).sum::<f64>() * 10.0).round() / 10.0;
            rows.push(T2StitchedMemberSplitPlanRow {
                split_plan_id: format!(
                    "T2STITCHEDSPLIT-{}-{}",
                    stable_id_fragment(&decision.decision_docket_id),
                    stable_id_fragment(&candidate_bundle_id)
                ),
                decision_docket_id: decision.decision_docket_id.clone(),
                route: decision.route.clone(),
                blocked_segment_bundle_id: decision.segment_bundle_id.clone(),
                candidate_segment_bundle_id: candidate_bundle_id,
                candidate_stitch_group_id: stitch_group_ids,
                state_scope,
                candidate_member_count: members.len(),
                candidate_length_miles: length_miles,
                split_action: "review-state-scoped-candidate-before-membership-mutation"
                    .to_string(),
                blocked_claims_before: decision.blocked_claims_after.clone(),
                blocked_claims_after: decision.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            });
        }
    }
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
