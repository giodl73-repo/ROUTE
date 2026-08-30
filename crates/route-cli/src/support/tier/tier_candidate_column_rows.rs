//! Helper `tier_candidate_column_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_column_rows(
    rows: &[TierContactWitnessInputRow],
    dispositions: &std::collections::HashMap<String, T2ClosureDisposition>,
    pavement_debt_index: &PavementDebtBudgetIndex,
    constraint_budget_index: &OptimizerConstraintBudgetIndex,
) -> Vec<TierCandidateColumnRow> {
    rows.iter()
        .map(|row| {
            let closure = dispositions.get(&canonical_route_key(&row.route));
            let column_decision = tier_candidate_column_decision(row, closure);
            let segment_bundle_id = closure
                .map(|closure| closure.segment_bundle_id.clone())
                .unwrap_or_default();
            let (
                pavement_debt_cost_m,
                pavement_debt_class,
                pavement_debt_basis,
                pavement_debt_artifact,
            ) = pavement_debt_for_candidate(&row.route, &segment_bundle_id, pavement_debt_index);
            let (
                hard_blocker_count,
                claim_blocker_count,
                constraint_debt_cost_m,
                lifecycle_debt_cost_m,
                constraint_penalty_score,
                top_constraint_classes,
                budget_qualification_effects,
                constraint_ledger_artifact,
            ) = constraint_budget_for_candidate(
                &row.route,
                &segment_bundle_id,
                constraint_budget_index,
            );
            let qualification_effects = merge_qualification_effects(
                &budget_qualification_effects,
                closure
                    .map(|closure| closure.qualification_effects.as_str())
                    .unwrap_or_default(),
            );
            TierCandidateColumnRow {
                tier: row.tier.clone(),
                route: row.route.clone(),
                candidate_type: "route-service-column".to_string(),
                graph_kind: "dual-route-graph".to_string(),
                split_objective: "route-mile-workload".to_string(),
                node_class: row.node_class.clone(),
                route_miles: row.route_miles,
                observed_t1_node_count: row.observed_t1_node_count,
                observed_dual_contacts: row.observed_dual_contacts,
                parent_trunks: row.observed_parent_trunks.clone(),
                component_id: row.component_id,
                component_route_count: row.component_route_count,
                component_status: row.component_status.clone(),
                witness_type: row.witness_type.clone(),
                repair_action: row.repair_action.clone(),
                repair_basis: closure
                    .filter(|closure| closure.disposition == "candidate-review")
                    .map(|closure| closure.basis.clone())
                    .unwrap_or_else(|| row.repair_basis.clone()),
                segment_bundle_id,
                bundle_status: closure
                    .map(|closure| closure.bundle_status.clone())
                    .unwrap_or_default(),
                bundle_action: closure
                    .map(|closure| closure.bundle_action.clone())
                    .unwrap_or_default(),
                pavement_debt_cost_m,
                pavement_debt_class,
                pavement_debt_basis,
                pavement_debt_artifact,
                hard_blocker_count,
                claim_blocker_count,
                constraint_debt_cost_m,
                lifecycle_debt_cost_m,
                constraint_penalty_score,
                top_constraint_classes,
                qualification_effects,
                constraint_ledger_artifact,
                column_decision: column_decision.to_string(),
                evidence_status: tier_candidate_column_evidence_status(row, closure),
                required_artifact: tier_candidate_column_required_artifact(row, closure),
                validation_status: if column_decision == "selected" {
                    "pass"
                } else {
                    "review"
                }
                .to_string(),
            }
        })
        .collect()
}
