//! Helper `constraint_budget_for_candidate`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn constraint_budget_for_candidate(
    route: &str,
    segment_bundle_id: &str,
    index: &OptimizerConstraintBudgetIndex,
) -> (usize, usize, f64, f64, f64, String, String, String) {
    if let Some(row) = index.by_bundle.get(segment_bundle_id) {
        return (
            row.hard_blocker_count,
            row.claim_blocker_count,
            row.constraint_debt_cost_m,
            row.lifecycle_debt_cost_m,
            row.constraint_penalty_score,
            row.top_constraint_classes.clone(),
            row.qualification_effects.clone(),
            row.constraint_ledger_artifact.clone(),
        );
    }
    if let Some(rollup) = index.by_route.get(&canonical_route_key(route)) {
        return (
            rollup.hard_blocker_count,
            rollup.claim_blocker_count,
            rollup.constraint_debt_cost_m,
            rollup.lifecycle_debt_cost_m,
            rollup.constraint_penalty_score,
            join_string_set(&rollup.top_constraint_classes),
            join_pipe_set(&rollup.qualification_effects),
            rollup.constraint_ledger_artifact.clone(),
        );
    }
    (
        0,
        0,
        0.0,
        0.0,
        0.0,
        "none".to_string(),
        String::new(),
        String::new(),
    )
}
