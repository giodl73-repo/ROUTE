//! Helper `t2_stitched_member_decision_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_decision_docket_gate_failures(
    rows: &[T2StitchedMemberDecisionDocketRow],
    scope_rows: &[T2StitchedMemberCandidateScopeReviewRow],
) -> Vec<String> {
    let expected = scope_rows
        .iter()
        .map(|row| row.scope_review_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if scope_rows.is_empty() {
        failures.push("stitched member decision docket has no scope rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member decision docket has {} rows but expected {} scope rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.decision_docket_id.trim().is_empty()
            || row.scope_review_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.candidate_state_scope.trim().is_empty()
            || row.decision.trim().is_empty()
            || row.decision_action.trim().is_empty()
            || row.repair_instruction.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete decision docket fields",
                row.route
            ));
        }
        if !seen.insert(row.scope_review_id.clone()) {
            failures.push(format!("{} appears more than once", row.scope_review_id));
        }
        if !expected.contains(row.scope_review_id.as_str()) {
            failures.push(format!(
                "{} is not a stitched scope row",
                row.scope_review_id
            ));
        }
        if !matches!(
            row.decision.as_str(),
            "split" | "merge" | "expand" | "manual-review"
        ) {
            failures.push(format!(
                "{} has invalid decision {}",
                row.route, row.decision
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} decision docket promoted readiness", row.route));
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
            failures.push(format!(
                "{expected_id} missing from stitched decision docket"
            ));
        }
    }
    failures
}
