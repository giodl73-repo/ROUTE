//! Helper `t4_terminal_access_column_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_column_rows(
    intake_rows: &[T3T4PressureIntakeRow],
    constraint_budget_index: &OptimizerConstraintBudgetIndex,
) -> Vec<T4TerminalAccessColumnRow> {
    let mut rows = intake_rows
        .iter()
        .filter(|row| row.current_tier == "T4" || row.intake_class == "t4-local-intake")
        .map(|row| {
            let zone_id = t3_zone_for_route(&row.route)
                .map(|(zone_id, _)| zone_id.to_string())
                .unwrap_or_else(|| "zone-assignment-needed".to_string());
            let (
                access_class,
                obligation,
                decision,
                evidence,
                map_treatment,
                basis,
                next,
                effect,
                status,
            ) = t4_terminal_access_decision(row, &zone_id);
            let (
                hard_blocker_count,
                claim_blocker_count,
                constraint_debt_cost_m,
                lifecycle_debt_cost_m,
                constraint_penalty_score,
                top_constraint_classes,
                _qualification_effects,
                constraint_ledger_artifact,
            ) = constraint_budget_for_candidate(&row.route, "", constraint_budget_index);
            T4TerminalAccessColumnRow {
                route: row.route.clone(),
                zone_id,
                current_score: row.current_score,
                constraint_adjusted_score: row.current_score - constraint_penalty_score,
                hard_blocker_count,
                claim_blocker_count,
                constraint_debt_cost_m,
                lifecycle_debt_cost_m,
                constraint_penalty_score,
                top_constraint_classes,
                constraint_ledger_artifact,
                access_class: access_class.to_string(),
                terminal_obligation: obligation.to_string(),
                promise_horizon_hours: 1,
                column_decision: decision.to_string(),
                evidence_required: evidence.to_string(),
                map_treatment: map_treatment.to_string(),
                selection_basis: basis.to_string(),
                source_artifact: row.source_artifact.clone(),
                next_artifact: next.to_string(),
                optimizer_effect: effect.to_string(),
                validation_status: status.to_string(),
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}
