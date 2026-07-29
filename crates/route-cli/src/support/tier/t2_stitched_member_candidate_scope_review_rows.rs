//! Helper `t2_stitched_member_candidate_scope_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_candidate_scope_review_rows(
    handoff_rows: &[T2StitchedMemberRegistryHandoffRow],
    candidate_rows: &[TierSegmentCandidateRow],
) -> Vec<T2StitchedMemberCandidateScopeReviewRow> {
    let mut rows = handoff_rows
        .iter()
        .map(|handoff| {
            let route_key = canonical_route_key(&handoff.route);
            let blocked_bundle_candidates = candidate_rows
                .iter()
                .filter(|row| {
                    row.segment_bundle_id == handoff.segment_bundle_id
                        && row.member_role == "stitched-member"
                })
                .collect::<Vec<_>>();
            let route_candidates = candidate_rows
                .iter()
                .filter(|row| {
                    canonical_route_key(&row.route) == route_key
                        && row.member_role == "stitched-member"
                })
                .collect::<Vec<_>>();
            let route_candidate_bundle_ids = route_candidates
                .iter()
                .map(|row| row.segment_bundle_id.clone())
                .collect::<std::collections::BTreeSet<_>>();
            let route_candidate_state_scope = route_candidates
                .iter()
                .filter_map(|row| {
                    let state = row.state.trim();
                    if state.is_empty() {
                        None
                    } else {
                        Some(state.to_string())
                    }
                })
                .collect::<std::collections::BTreeSet<_>>();
            let scope_action = if route_candidate_bundle_ids.len() > 1 {
                "review-route-family-state-scope-before-member-expansion"
            } else {
                "review-single-bundle-member-expansion-before-replay"
            };
            T2StitchedMemberCandidateScopeReviewRow {
                scope_review_id: format!(
                    "T2STITCHEDSCOPE-{}",
                    stable_id_fragment(&handoff.handoff_id)
                ),
                handoff_id: handoff.handoff_id.clone(),
                route: handoff.route.clone(),
                segment_bundle_id: handoff.segment_bundle_id.clone(),
                blocked_bundle_candidate_count: blocked_bundle_candidates.len(),
                route_candidate_count: route_candidates.len(),
                route_candidate_bundle_count: route_candidate_bundle_ids.len(),
                route_candidate_state_scope: route_candidate_state_scope
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(";"),
                route_candidate_bundle_ids: route_candidate_bundle_ids
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(";"),
                scope_decision: "held-for-scope-review".to_string(),
                scope_action: scope_action.to_string(),
                blocked_claims_before: handoff.blocked_claims_after.clone(),
                blocked_claims_after: handoff.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: "data/tier-segment-candidates.csv".to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

