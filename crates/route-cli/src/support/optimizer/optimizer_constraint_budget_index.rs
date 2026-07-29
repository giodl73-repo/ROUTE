//! Helper `optimizer_constraint_budget_index`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_budget_index(
    rows: &[OptimizerConstraintBudgetRow],
) -> OptimizerConstraintBudgetIndex {
    let mut index = OptimizerConstraintBudgetIndex::default();
    for row in rows {
        if !row.segment_bundle_id.trim().is_empty() {
            index
                .by_bundle
                .insert(row.segment_bundle_id.clone(), row.clone());
        }
        if !row.route.trim().is_empty() {
            let rollup = index
                .by_route
                .entry(canonical_route_key(&row.route))
                .or_default();
            rollup.hard_blocker_count += row.hard_blocker_count;
            rollup.claim_blocker_count += row.claim_blocker_count;
            rollup.constraint_debt_cost_m =
                round_cost_m(rollup.constraint_debt_cost_m + row.constraint_debt_cost_m);
            rollup.lifecycle_debt_cost_m =
                round_cost_m(rollup.lifecycle_debt_cost_m + row.lifecycle_debt_cost_m);
            rollup.constraint_penalty_score =
                round_cost_m(rollup.constraint_penalty_score + row.constraint_penalty_score);
            for class in row.top_constraint_classes.split('|').map(str::trim) {
                if !class.is_empty() {
                    rollup.top_constraint_classes.insert(class.to_string());
                }
            }
            for effect in row.qualification_effects.split('|').map(str::trim) {
                if !effect.is_empty() {
                    rollup.qualification_effects.insert(effect.to_string());
                }
            }
            if rollup.constraint_ledger_artifact.is_empty() {
                rollup.constraint_ledger_artifact = row.constraint_ledger_artifact.clone();
            }
        }
    }
    index
}

