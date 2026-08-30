//! Helper `tier_candidate_column_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_column_gate_failures(rows: &[TierCandidateColumnRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no candidate column rows emitted".to_string());
        return failures;
    }
    let selected = rows
        .iter()
        .filter(|row| row.column_decision == "selected")
        .count();
    if selected == 0 {
        failures.push("no selected candidate columns available".to_string());
    }
    for row in rows {
        if row.column_decision == "selected" && !row.validation_status.eq_ignore_ascii_case("pass")
        {
            failures.push(format!("{} selected without passing validation", row.route));
        }
        if row.pavement_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative pavement debt cost", row.route));
        }
        if row.pavement_debt_cost_m > 0.0
            && (row.pavement_debt_class.trim().is_empty()
                || row.pavement_debt_artifact.trim().is_empty()
                || row.pavement_debt_basis.trim().is_empty())
        {
            failures.push(format!(
                "{} has pavement debt cost without debt class, basis, and artifact",
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
