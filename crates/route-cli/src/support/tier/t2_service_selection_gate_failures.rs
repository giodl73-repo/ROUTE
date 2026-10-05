//! Helper `t2_service_selection_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_selection_gate_failures(rows: &[T2ServiceSelectionRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 service selection rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.treatment_status == "selected-treatment"
            && row.beck_corridor.is_empty()
            && row.selection_action != "source-needed"
        {
            failures.push(format!("{} missing Beck T2 diagnostic", row.route));
        }
        if row.treatment_status == "selected-treatment"
            && !matches!(
                row.selection_action.as_str(),
                "keep-service-column" | "source-needed"
            )
        {
            failures.push(format!(
                "{} selected treatment requires {} before keep",
                row.route, row.selection_action
            ));
        }
        if row.selection_action == "keep-service-column"
            && (row.duplicate_service_count > 0
                || row.close_parallel_count > 0
                || row.unstopped_t1_contact_count > 0)
        {
            failures.push(format!(
                "{} kept despite unresolved T2 diagnostic",
                row.route
            ));
        }
        if !row.beck_service_action.is_empty()
            && t2_qualification_action_for(&row.beck_service_action, &row.qualification_basis)
                .is_none()
        {
            failures.push(format!(
                "{} has uncovered T2 qualification action/basis {} {}",
                row.route, row.beck_service_action, row.qualification_basis
            ));
        }
        if !row.beck_service_action.is_empty()
            && (row.qualification_map_treatment.trim().is_empty()
                || row.qualification_gate_policy.trim().is_empty()
                || row.qualification_game_use.trim().is_empty())
        {
            failures.push(format!(
                "{} missing T2 qualification action treatment columns",
                row.route
            ));
        }
        if row.constraint_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative constraint debt cost", row.route));
        }
        if row.constraint_penalty_score < 0.0 {
            failures.push(format!("{} has negative constraint penalty", row.route));
        }
        if (row.constraint_debt_cost_m > 0.0
            || row.hard_blocker_count > 0
            || row.claim_blocker_count > 0
            || row.constraint_penalty_score > 0.0)
            && (row.top_constraint_classes.trim().is_empty()
                || row.constraint_ledger_artifact.trim().is_empty())
        {
            failures.push(format!(
                "{} has constraint pressure without class summary and ledger artifact",
                row.route
            ));
        }
    }
    failures
}
