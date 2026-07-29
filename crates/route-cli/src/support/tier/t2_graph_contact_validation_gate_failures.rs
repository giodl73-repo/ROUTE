//! Helper `t2_graph_contact_validation_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_graph_contact_validation_gate_failures(rows: &[T2GraphContactValidationRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_graph_contacts__" {
        let row = &rows[0];
        if row.contact_action != "graph-contact-clear" || row.validation_status != "pass" {
            failures.push("graph contact clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.contact_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete graph contact validation",
                row.route
            ));
        }
    }
    failures
}

