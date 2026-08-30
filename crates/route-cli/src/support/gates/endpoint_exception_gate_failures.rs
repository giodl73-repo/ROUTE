//! Helper `endpoint_exception_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn endpoint_exception_gate_failures(
    rows: &[&EndpointExceptionRow],
    require_terminal_worthy: bool,
) -> Vec<String> {
    let mut failures = Vec::new();
    for row in rows {
        let route = normalise_designation(&row.route);
        if route.is_empty() {
            failures.push("row missing route".to_string());
        }
        if row.requested_tier.trim().is_empty() {
            failures.push(format!("{route}: missing requested_tier"));
        }
        if !endpoint_exception_has_contract(row) {
            failures.push(format!("{route}: incomplete endpoint exception contract"));
        }
        if !valid_endpoint_evidence_level(&row.evidence_level) {
            failures.push(format!(
                "{route}: unsupported evidence_level {}",
                row.evidence_level
            ));
        }
        if require_terminal_worthy && !endpoint_exception_is_terminal_worthy(row) {
            failures.push(format!(
                "{route}: endpoint exception is not terminal-worthy for requested tier"
            ));
        }
    }
    failures
}
