//! Helper `t4_terminal_access_column_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_column_gate_failures(rows: &[T4TerminalAccessColumnRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T4 terminal access columns emitted".to_string());
        return failures;
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.access_class.trim().is_empty()
            || row.terminal_obligation.trim().is_empty()
            || row.promise_horizon_hours != 1
            || row.column_decision.trim().is_empty()
            || row.evidence_required.trim().is_empty()
            || row.map_treatment.trim().is_empty()
            || row.selection_basis.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete T4 access column", row.route));
        }
        if !seen.insert(canonical_route_key(&row.route)) {
            failures.push(format!("{} has duplicate T4 access column", row.route));
        }
        if row.column_decision == "selected-local-access" && row.zone_id == "zone-assignment-needed"
        {
            failures.push(format!(
                "{} selected local access without a zone assignment",
                row.route
            ));
        }
        if row.constraint_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative constraint debt cost", row.route));
        }
        if row.lifecycle_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative lifecycle debt cost", row.route));
        }
        if row.constraint_penalty_score < 0.0 {
            failures.push(format!("{} has negative constraint penalty", row.route));
        }
        if (row.hard_blocker_count > 0
            || row.claim_blocker_count > 0
            || row.constraint_debt_cost_m > 0.0
            || row.lifecycle_debt_cost_m > 0.0
            || row.constraint_penalty_score > 0.0)
            && (row.top_constraint_classes.trim().is_empty()
                || row.constraint_ledger_artifact.trim().is_empty())
        {
            failures.push(format!(
                "{} has constraint pressure without class summary and ledger artifact",
                row.route
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.route, row.validation_status
            ));
        }
    }
    failures
}

