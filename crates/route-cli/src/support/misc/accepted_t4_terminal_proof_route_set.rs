//! Helper `accepted_t4_terminal_proof_route_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn accepted_t4_terminal_proof_route_set(
    rows: &[T4TerminalContactDistrictProofImportRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.import_status == "accepted"
                && row.proof_decision == "source-backed"
                && row.validation_status == "pass"
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}
