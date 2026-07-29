//! Helper `t2_terminal_contact_validation_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_terminal_contact_validation_gate_failures(
    rows: &[T2TerminalContactValidationRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_terminal_contacts__" {
        let row = &rows[0];
        if row.terminal_action != "terminal-contact-clear" || row.validation_status != "pass" {
            failures.push("terminal contact clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.held_action_type.trim().is_empty()
            || row.terminal_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete terminal contact validation",
                row.route
            ));
        }
    }
    failures
}

