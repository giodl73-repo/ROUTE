//! Helper `pavement_debt_budget_index`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_debt_budget_index(rows: &[TierPavementDebtBudgetRow]) -> PavementDebtBudgetIndex {
    let mut index = PavementDebtBudgetIndex::default();
    for row in rows {
        index
            .by_bundle
            .insert(row.segment_bundle_id.clone(), row.clone());
        let route_rollup = index
            .by_route
            .entry(canonical_route_key(&row.route))
            .or_default();
        route_rollup.total_debt_cost_m =
            round_cost_m(route_rollup.total_debt_cost_m + row.total_debt_cost_m);
        route_rollup.debt_classes.insert(row.debt_class.clone());
        route_rollup
            .affected_bundles
            .insert(row.segment_bundle_id.clone());
    }
    index
}

