//! Helper `t2_contact_closure_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_contact_closure_gate_failures(rows: &[T2ContactClosureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_contact_closures__" {
        let row = &rows[0];
        if row.contact_action != "contact-closure-clear" || row.validation_status != "pass" {
            failures.push("contact closure clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.blocker_class.trim().is_empty()
            || row.contact_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete contact closure", row.route));
        }
    }
    failures
}
