//! Helper `t2_stitched_member_split_plan_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_split_plan_gate_failures(
    rows: &[T2StitchedMemberSplitPlanRow],
    decision_rows: &[T2StitchedMemberDecisionDocketRow],
) -> Vec<String> {
    let split_decisions = decision_rows
        .iter()
        .filter(|row| row.decision == "split" && row.validation_status == "review")
        .collect::<Vec<_>>();
    let expected = split_decisions
        .iter()
        .map(|row| row.decision_docket_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut counts_by_decision = std::collections::BTreeMap::<&str, usize>::new();
    let mut failures = Vec::new();
    if split_decisions.is_empty() {
        failures.push("stitched member split plan has no split decisions".to_string());
    }
    if rows.is_empty() {
        failures.push("stitched member split plan has no rows".to_string());
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        *counts_by_decision
            .entry(row.decision_docket_id.as_str())
            .or_default() += 1;
        if row.split_plan_id.trim().is_empty()
            || row.decision_docket_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.blocked_segment_bundle_id.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.candidate_stitch_group_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.split_action.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete split plan fields", row.route));
        }
        if !seen.insert(row.split_plan_id.clone()) {
            failures.push(format!("{} appears more than once", row.split_plan_id));
        }
        if !expected.contains(row.decision_docket_id.as_str()) {
            failures.push(format!(
                "{} is not a split decision docket row",
                row.decision_docket_id
            ));
        }
        if row.candidate_member_count == 0 || row.candidate_length_miles <= 0.0 {
            failures.push(format!(
                "{} {} has no usable candidate members",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if row.validation_status != "review"
            || row.split_action.contains("pass")
            || row.split_action.contains("bound")
        {
            failures.push(format!("{} split plan promoted readiness", row.route));
        }
        if row.blocked_claims_before != "game;incident;publication;upgrade"
            || row.blocked_claims_after != "game;incident;publication;upgrade"
            || row.blocker_delta != 0
        {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
    }
    for decision in split_decisions {
        let actual = counts_by_decision
            .get(decision.decision_docket_id.as_str())
            .copied()
            .unwrap_or_default();
        if actual != decision.candidate_bundle_count {
            failures.push(format!(
                "{} has {} split rows but expected {} candidate bundles",
                decision.route, actual, decision.candidate_bundle_count
            ));
        }
    }
    failures
}
