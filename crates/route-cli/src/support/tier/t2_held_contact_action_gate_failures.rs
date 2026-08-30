//! Helper `t2_held_contact_action_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_held_contact_action_gate_failures(rows: &[T2HeldContactActionRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no held T2 contact action rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.held_action_type.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete held contact action", row.route));
        }
    }
    failures
}
