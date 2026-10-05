//! Helper `t2_service_selection_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_selection_rows(
    regionalizer_rows: &[T2RegionalizerRow],
    diagnostics: &[route_map::BeckT2DiagnosticRow],
) -> Vec<T2ServiceSelectionRow> {
    let diagnostic_by_route = diagnostics
        .iter()
        .map(|row| (canonical_route_key(row.corridor), row))
        .collect::<std::collections::HashMap<_, _>>();

    regionalizer_rows
        .iter()
        .map(|row| {
            let diagnostic = diagnostic_by_route.get(&canonical_route_key(&row.route));
            let qualification_action = diagnostic.and_then(|diag| {
                t2_qualification_action_for(diag.service_action, diag.qualification_basis)
            });
            let (selection_action, selection_basis, validation_status) =
                t2_service_selection_decision(row, diagnostic.copied());

            T2ServiceSelectionRow {
                tier: row.tier.clone(),
                region_id: row.region_id.clone(),
                route: row.route.clone(),
                parent_trunks: row.parent_trunks.clone(),
                column_decision: row.column_decision.clone(),
                treatment_status: row.treatment_status.clone(),
                beck_corridor: diagnostic
                    .map(|diag| diag.corridor.to_string())
                    .unwrap_or_default(),
                beck_service_class: diagnostic
                    .map(|diag| diag.service_class.to_string())
                    .unwrap_or_default(),
                beck_color_mode: diagnostic
                    .map(|diag| diag.color_mode.to_string())
                    .unwrap_or_default(),
                beck_start_trunk: diagnostic
                    .map(|diag| diag.start_trunk.to_string())
                    .unwrap_or_default(),
                beck_end_trunk: diagnostic
                    .map(|diag| diag.end_trunk.to_string())
                    .unwrap_or_default(),
                duplicate_service_count: diagnostic
                    .map(|diag| diag.duplicate_service_count)
                    .unwrap_or_default(),
                duplicate_service_corridors: diagnostic
                    .map(|diag| diag.duplicate_service_corridors.clone())
                    .unwrap_or_default(),
                close_parallel_count: diagnostic
                    .map(|diag| diag.close_parallel_count)
                    .unwrap_or_default(),
                close_parallel_corridors: diagnostic
                    .map(|diag| diag.close_parallel_corridors.clone())
                    .unwrap_or_default(),
                unstopped_t1_contact_count: diagnostic
                    .map(|diag| diag.unstopped_t1_contact_count)
                    .unwrap_or_default(),
                unstopped_t1_contacts: diagnostic
                    .map(|diag| diag.unstopped_t1_contacts.clone())
                    .unwrap_or_default(),
                pavement_debt_cost_m: row.pavement_debt_cost_m,
                pavement_debt_class: row.pavement_debt_class.clone(),
                pavement_debt_basis: row.pavement_debt_basis.clone(),
                hard_blocker_count: row.hard_blocker_count,
                claim_blocker_count: row.claim_blocker_count,
                constraint_debt_cost_m: row.constraint_debt_cost_m,
                lifecycle_debt_cost_m: row.lifecycle_debt_cost_m,
                constraint_penalty_score: row.constraint_penalty_score,
                top_constraint_classes: row.top_constraint_classes.clone(),
                qualification_effects: row.qualification_effects.clone(),
                constraint_ledger_artifact: row.constraint_ledger_artifact.clone(),
                beck_service_action: diagnostic
                    .map(|diag| diag.service_action.to_string())
                    .unwrap_or_default(),
                qualification_basis: diagnostic
                    .map(|diag| diag.qualification_basis.to_string())
                    .unwrap_or_default(),
                qualification_map_treatment: qualification_action
                    .as_ref()
                    .map(|action| action.map_treatment.to_string())
                    .unwrap_or_default(),
                qualification_gate_policy: qualification_action
                    .as_ref()
                    .map(|action| action.gate_policy.to_string())
                    .unwrap_or_default(),
                qualification_game_use: qualification_action
                    .as_ref()
                    .map(|action| action.game_use.to_string())
                    .unwrap_or_default(),
                selection_action,
                selection_basis,
                validation_status,
            }
        })
        .collect()
}
