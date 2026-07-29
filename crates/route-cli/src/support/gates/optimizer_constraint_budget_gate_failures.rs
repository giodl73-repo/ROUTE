//! Helper `optimizer_constraint_budget_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_budget_gate_failures(
    rows: &[OptimizerConstraintBudgetRow],
    ledger_rows: &[OptimizerConstraintLedgerRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if ledger_rows.is_empty() {
        failures.push("optimizer constraint ledger is empty".to_string());
    }
    if rows.is_empty() {
        failures.push("no optimizer constraint budget rows emitted".to_string());
        return failures;
    }
    let rolled_up_count = rows.iter().map(|row| row.ledger_row_count).sum::<usize>();
    if rolled_up_count != ledger_rows.len() {
        failures.push(format!(
            "budget rows roll up {} ledger rows, expected {}",
            rolled_up_count,
            ledger_rows.len()
        ));
    }
    let mut ids = std::collections::BTreeSet::new();
    for row in rows {
        if !ids.insert(row.budget_id.as_str()) {
            failures.push(format!("duplicate budget id {}", row.budget_id));
        }
        if row.budget_id.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.subject_scope.trim().is_empty()
            || row.subject_id.trim().is_empty()
            || row.top_constraint_classes.trim().is_empty()
            || row.next_artifacts.trim().is_empty()
            || row.constraint_ledger_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete budget row", row.budget_id));
        }
        if row.ledger_row_count == 0 {
            failures.push(format!("{} has zero ledger rows", row.budget_id));
        }
        if row.subject_scope == "bundle" && row.segment_bundle_id.trim().is_empty() {
            failures.push(format!("{} bundle row lacks bundle id", row.budget_id));
        }
        if row.budget_debt_count > 0 && row.constraint_debt_cost_m <= 0.0 {
            failures.push(format!(
                "{} has debt rows without positive debt cost",
                row.budget_id
            ));
        }
        if row.claim_blocker_count > 0 && row.blocking_claims.trim().is_empty() {
            failures.push(format!(
                "{} has claim blockers without blocking claims",
                row.budget_id
            ));
        }
        if row
            .top_constraint_classes
            .contains("game_ops_bundle_binding_relief")
            && ledger_rows.iter().any(|ledger| {
                ledger.segment_bundle_id == row.segment_bundle_id
                    && ledger
                        .optimizer_effect
                        .contains("qualification_gate_policy=")
            })
            && row.qualification_effects.trim().is_empty()
        {
            failures.push(format!(
                "{} drops qualification-bearing optimizer effects",
                row.budget_id
            ));
        }
    }
    failures
}

