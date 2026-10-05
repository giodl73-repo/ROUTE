//! Helper `optimizer_residual_blocker_backlog_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_residual_blocker_backlog_gate_failures(
    rows: &[OptimizerResidualBlockerBacklogRow],
    budget_rows: &[OptimizerConstraintBudgetRow],
) -> Vec<String> {
    let residual_rows = budget_rows
        .iter()
        .filter(|row| row.validation_status != "pass")
        .collect::<Vec<_>>();
    let mut failures = Vec::new();
    if residual_rows.is_empty() {
        failures.push("constraint budget has no residual blocker rows".to_string());
    }
    if rows.is_empty() {
        failures.push("residual blocker backlog emitted no rows".to_string());
        return failures;
    }
    let expected_hard = residual_rows
        .iter()
        .map(|row| row.hard_blocker_count)
        .sum::<usize>();
    let expected_claim = residual_rows
        .iter()
        .map(|row| row.claim_blocker_count)
        .sum::<usize>();
    let expected_debt = residual_rows
        .iter()
        .map(|row| row.budget_debt_count)
        .sum::<usize>();
    let actual_hard = rows
        .iter()
        .map(|row| row.total_hard_blockers)
        .sum::<usize>();
    let actual_claim = rows
        .iter()
        .map(|row| row.total_claim_blockers)
        .sum::<usize>();
    let actual_debt = rows
        .iter()
        .map(|row| row.total_budget_debt_count)
        .sum::<usize>();
    if (actual_hard, actual_claim, actual_debt) != (expected_hard, expected_claim, expected_debt) {
        failures.push(format!(
            "backlog totals hard/claim/debt = {actual_hard}/{actual_claim}/{actual_debt}, expected {expected_hard}/{expected_claim}/{expected_debt}"
        ));
    }
    let mut ids = std::collections::BTreeSet::<&str>::new();
    for row in rows {
        if !ids.insert(row.backlog_id.as_str()) {
            failures.push(format!("duplicate backlog id {}", row.backlog_id));
        }
        if row.backlog_id.trim().is_empty()
            || row.priority_class.trim().is_empty()
            || row.blocker_family.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.representative_subjects.trim().is_empty()
            || row.next_artifacts.trim().is_empty()
            || row.backlog_decision.trim().is_empty()
            || row.next_wave.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete backlog fields", row.backlog_id));
        }
        if row.backlog_decision != "triage-only-no-blocker-relief"
            || row.validation_status != "review"
        {
            failures.push(format!("{} promotes residual blockers", row.backlog_id));
        }
    }
    failures
}
