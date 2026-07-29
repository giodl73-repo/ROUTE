//! Helper `rejected_t4_terminal_proof_route_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn rejected_t4_terminal_proof_route_set(
    rows: &[T4TerminalContactRejectedProofSourceRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.rejection_status == "route-not-supported-by-terminal-access-source"
                && row.validation_status == "pass"
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

