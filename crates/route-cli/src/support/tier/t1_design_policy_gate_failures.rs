//! Helper `t1_design_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_design_policy_gate_failures(
    review_rows: &[T1DesignReviewCsvRow],
    policy_rows: &[T1DesignPolicyActionRow],
) -> Vec<String> {
    let actions = policy_rows
        .iter()
        .map(|row| row.action.trim().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if policy_rows.is_empty() {
        failures.push("no T1 design policy action rows".to_string());
    }
    for row in policy_rows {
        if row.action.trim().is_empty()
            || row.applies_to_status.trim().is_empty()
            || row.definition.trim().is_empty()
            || row.required_policy.trim().is_empty()
            || row.design_treatment.trim().is_empty()
            || row.gate_policy.trim().is_empty()
            || row.next_selector_use.trim().is_empty()
        {
            failures.push(format!("{} has incomplete policy contract", row.action));
        }
    }
    for row in review_rows {
        if !actions.contains(row.next_design_action.trim()) {
            failures.push(format!(
                "{} uses uncovered next_design_action {}",
                row.route, row.next_design_action
            ));
        }
        if row.selected
            && row.design_status == "policy-review"
            && row.next_design_action.trim().is_empty()
        {
            failures.push(format!("{} policy review has no next action", row.route));
        }
    }
    failures
}

