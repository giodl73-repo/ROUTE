//! Helper `endpoint_exception_is_terminal_worthy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn endpoint_exception_is_terminal_worthy(row: &EndpointExceptionRow) -> bool {
    if !endpoint_exception_has_contract(row) {
        return false;
    }

    let role = row.endpoint_role.trim().to_ascii_lowercase();
    let exception_type = row.exception_type.trim().to_ascii_lowercase();
    let terminal_role = matches!(
        role.as_str(),
        "national_terminal" | "t2_terminal_exception" | "graph_endpoint_gap"
    );
    let terminal_exception = matches!(
        exception_type.as_str(),
        "port_terminal"
            | "border_gateway"
            | "military_logistics"
            | "resilience_relief"
            | "future_tier_continuation"
            | "regional_terminal"
    );
    terminal_role && terminal_exception
}

