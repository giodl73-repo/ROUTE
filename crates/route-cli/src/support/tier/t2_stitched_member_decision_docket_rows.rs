//! Helper `t2_stitched_member_decision_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_decision_docket_rows(
    scope_rows: &[T2StitchedMemberCandidateScopeReviewRow],
) -> Vec<T2StitchedMemberDecisionDocketRow> {
    let mut rows = scope_rows
        .iter()
        .map(|scope| {
            let decision = if scope.route_candidate_bundle_count > 1 {
                "split"
            } else if scope.blocked_bundle_candidate_count >= 2 {
                "expand"
            } else {
                "manual-review"
            };
            let (decision_action, repair_instruction) = match decision {
                "split" => (
                    "split-route-family-scope-before-member-expansion",
                    "choose the state-scoped bundle ids that belong to the blocked stitched service before any merge or expansion",
                ),
                "expand" => (
                    "expand-blocked-bundle-members-after-scope-confirmation",
                    "append vetted candidate members to the blocked bundle only after the route family scope is explicit",
                ),
                _ => (
                    "manual-scope-review-before-member-expansion",
                    "hold until a human review selects split, merge, or expand",
                ),
            };
            T2StitchedMemberDecisionDocketRow {
                decision_docket_id: format!(
                    "T2STITCHEDDECISION-{}",
                    stable_id_fragment(&scope.scope_review_id)
                ),
                scope_review_id: scope.scope_review_id.clone(),
                route: scope.route.clone(),
                segment_bundle_id: scope.segment_bundle_id.clone(),
                candidate_bundle_count: scope.route_candidate_bundle_count,
                candidate_state_scope: scope.route_candidate_state_scope.clone(),
                decision: decision.to_string(),
                decision_action: decision_action.to_string(),
                repair_instruction: repair_instruction.to_string(),
                blocked_claims_before: scope.blocked_claims_after.clone(),
                blocked_claims_after: scope.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: "data/tier-segment-candidates.csv".to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
