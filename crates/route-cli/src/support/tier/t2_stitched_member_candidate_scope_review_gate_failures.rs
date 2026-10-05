//! Helper `t2_stitched_member_candidate_scope_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_candidate_scope_review_gate_failures(
    rows: &[T2StitchedMemberCandidateScopeReviewRow],
    handoff_rows: &[T2StitchedMemberRegistryHandoffRow],
) -> Vec<String> {
    let expected = handoff_rows
        .iter()
        .map(|row| row.handoff_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member candidate scope review has {} rows but expected {} handoff rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.scope_review_id.trim().is_empty()
            || row.handoff_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.route_candidate_state_scope.trim().is_empty()
            || row.route_candidate_bundle_ids.trim().is_empty()
            || row.scope_decision.trim().is_empty()
            || row.scope_action.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete scope review fields", row.route));
        }
        if !seen.insert(row.handoff_id.clone()) {
            failures.push(format!("{} appears more than once", row.handoff_id));
        }
        if !expected.contains(row.handoff_id.as_str()) {
            failures.push(format!("{} is not a stitched handoff row", row.handoff_id));
        }
        if row.scope_decision == "pass"
            || row.scope_decision == "bound"
            || row.validation_status != "review"
        {
            failures.push(format!("{} scope review promoted readiness", row.route));
        }
        if row.route_candidate_count == 0
            || row.route_candidate_bundle_count == 0
            || row.blocked_bundle_candidate_count == 0
        {
            failures.push(format!("{} lacks candidate evidence counts", row.route));
        }
        if row.blocked_claims_before != "game;incident;publication;upgrade"
            || row.blocked_claims_after != "game;incident;publication;upgrade"
            || row.blocker_delta != 0
        {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from candidate scope review"));
        }
    }
    failures
}
