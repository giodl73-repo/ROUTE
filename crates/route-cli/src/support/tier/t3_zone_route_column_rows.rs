//! Helper `t3_zone_route_column_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_route_column_rows(
    obligations: &[T3ZoneAccessObligationRow],
    intake_rows: &[T3T4PressureIntakeRow],
    constraint_budget_index: &OptimizerConstraintBudgetIndex,
) -> Vec<T3ZoneRouteColumnRow> {
    let intake_by_route = intake_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::BTreeMap<_, _>>();

    let mut rows = Vec::new();
    for obligation in obligations {
        for route in semicolon_values(&obligation.candidate_routes) {
            let intake = intake_by_route.get(&canonical_route_key(&route));
            let current_tier = intake
                .map(|row| row.current_tier.clone())
                .unwrap_or_else(|| "unknown".to_string());
            let current_score = intake.map(|row| row.current_score).unwrap_or(0.0);
            let (
                hard_blocker_count,
                claim_blocker_count,
                constraint_debt_cost_m,
                lifecycle_debt_cost_m,
                constraint_penalty_score,
                top_constraint_classes,
                _qualification_effects,
                constraint_ledger_artifact,
            ) = constraint_budget_for_candidate(&route, "", constraint_budget_index);
            let constraint_adjusted_score = current_score - constraint_penalty_score;
            let (
                column_decision,
                zone_role,
                contact_requirement,
                map_treatment,
                basis,
                next_artifact,
                effect,
                status,
            ) = t3_zone_route_column_decision(obligation, &route, current_score, intake.is_some());
            rows.push(T3ZoneRouteColumnRow {
                zone_id: obligation.zone_id.clone(),
                zone_name: obligation.zone_name.clone(),
                obligation_class: obligation.obligation_class.clone(),
                route,
                current_tier,
                current_score,
                constraint_adjusted_score,
                hard_blocker_count,
                claim_blocker_count,
                constraint_debt_cost_m,
                lifecycle_debt_cost_m,
                constraint_penalty_score,
                top_constraint_classes,
                constraint_ledger_artifact,
                promise_horizon_hours: obligation.promise_horizon_hours,
                column_decision: column_decision.to_string(),
                zone_role: zone_role.to_string(),
                contact_requirement: contact_requirement.to_string(),
                map_treatment: map_treatment.to_string(),
                selection_basis: basis.to_string(),
                source_obligation: obligation.access_target.clone(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: effect.to_string(),
                validation_status: status.to_string(),
            });
        }
    }

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| a.obligation_class.cmp(&b.obligation_class))
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}

