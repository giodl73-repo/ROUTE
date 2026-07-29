//! Helper `t2_game_ops_binding_intake_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_binding_intake_gate_failures(
    rows: &[T2GameOpsBindingIntakeRow],
    budget_rows: &[OptimizerConstraintBudgetRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = budget_rows
        .iter()
        .filter(|row| {
            row.tier == "T2"
                && constraint_class_values(&row.top_constraint_classes)
                    .iter()
                    .any(|class| class == "game_ops_bundle_binding")
        })
        .map(|row| row.budget_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if rows.is_empty() {
        failures.push("no T2 game/ops binding intake rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game/ops binding intake has {} rows but expected {} budget blockers",
            rows.len(),
            expected.len()
        ));
    }
    let budget_by_id = budget_rows
        .iter()
        .map(|budget| (budget.budget_id.as_str(), budget))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.intake_id.trim().is_empty()
            || row.budget_id.trim().is_empty()
            || row.subject_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.top_constraint_classes.trim().is_empty()
            || row.next_artifacts.trim().is_empty()
            || row.constraint_ledger_artifact.trim().is_empty()
            || row.intake_status.trim().is_empty()
            || row.decision_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete intake fields", row.intake_id));
        }
        if !seen.insert(row.budget_id.clone()) {
            failures.push(format!("{} appears more than once", row.budget_id));
        }
        if !expected.contains(row.budget_id.as_str()) {
            failures.push(format!(
                "{} is not a T2 game/ops binding budget blocker",
                row.budget_id
            ));
        }
        if !constraint_class_values(&row.top_constraint_classes)
            .iter()
            .any(|class| class == "game_ops_bundle_binding")
        {
            failures.push(format!(
                "{} lacks game_ops_bundle_binding class",
                row.budget_id
            ));
        }
        if row.claim_blocker_count == 0 {
            failures.push(format!("{} lacks claim blocker count", row.budget_id));
        }
        if row.intake_status != "decision-needed" || row.validation_status != "review" {
            failures.push(format!("{} intake status is not review", row.budget_id));
        }
        if let Some(budget) = budget_by_id.get(row.budget_id.as_str()) {
            if !budget.qualification_effects.trim().is_empty()
                && row.qualification_effects.trim().is_empty()
            {
                failures.push(format!(
                    "{} drops qualification effects from budget row",
                    row.budget_id
                ));
            }
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from T2 game/ops binding intake"
            ));
        }
    }
    failures
}

