//! Helper `t2_stitched_member_selection_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_selection_docket_gate_failures(
    rows: &[T2StitchedMemberSelectionDocketRow],
    split_rows: &[T2StitchedMemberSplitPlanRow],
) -> Vec<String> {
    let expected = split_rows
        .iter()
        .map(|row| row.split_plan_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if split_rows.is_empty() {
        failures.push("stitched member selection docket has no split rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member selection docket has {} rows but expected {} split rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.selection_docket_id.trim().is_empty()
            || row.split_plan_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.blocked_segment_bundle_id.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.selection_decision.trim().is_empty()
            || row.selection_action.trim().is_empty()
            || row.evidence_requirement.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete selection fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.split_plan_id.clone()) {
            failures.push(format!("{} appears more than once", row.split_plan_id));
        }
        if !expected.contains(row.split_plan_id.as_str()) {
            failures.push(format!("{} is not a split plan row", row.split_plan_id));
        }
        if row.selection_decision != "evidence-needed"
            || row.selection_action.contains("selected")
            || row.selection_action.contains("reject")
            || row.selection_action.contains("pass")
            || row.selection_action.contains("bound")
            || row.validation_status != "review"
        {
            failures.push(format!("{} selection docket promoted readiness", row.route));
        }
        if row.candidate_member_count == 0 || row.candidate_length_miles <= 0.0 {
            failures.push(format!(
                "{} {} has no usable candidate members",
                row.route, row.candidate_segment_bundle_id
            ));
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
            failures.push(format!("{expected_id} missing from selection docket"));
        }
    }
    failures
}
