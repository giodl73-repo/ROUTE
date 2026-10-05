//! Helper `pavement_debt_for_candidate`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_debt_for_candidate(
    route: &str,
    segment_bundle_id: &str,
    index: &PavementDebtBudgetIndex,
) -> (f64, String, String, String) {
    if let Some(row) = index.by_bundle.get(segment_bundle_id) {
        return (
            row.total_debt_cost_m,
            row.debt_class.clone(),
            row.budget_basis.clone(),
            "data/tier-pavement-debt-budget.csv".to_string(),
        );
    }

    if let Some(rollup) = index.by_route.get(&canonical_route_key(route)) {
        return (
            rollup.total_debt_cost_m,
            join_string_set(&rollup.debt_classes),
            format!(
                "route-level pavement debt rollup across {} bundle(s) pending candidate bundle materialization",
                rollup.affected_bundles.len()
            ),
            "data/tier-pavement-debt-budget.csv".to_string(),
        );
    }

    (
        0.0,
        "none".to_string(),
        "no pavement debt row joined".to_string(),
        String::new(),
    )
}
