//! Helper `endpoint_exception_failure_reason`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn endpoint_exception_failure_reason(
    row: &route_network::TierConnectivityRow,
    exceptions: &[&EndpointExceptionRow],
) -> String {
    if exceptions.is_empty() {
        return "no endpoint exception record".to_string();
    }

    if matches!(
        row.classification,
        route_network::TierNodeClass::MissingGraphData
    ) {
        return "graph/contact data must be fixed before endpoint exception can promote route"
            .to_string();
    }

    let invalid_contracts = exceptions
        .iter()
        .filter(|exception| !endpoint_exception_has_contract(exception))
        .count();
    if invalid_contracts > 0 {
        return format!(
            "{invalid_contracts} endpoint exception record(s) lack a complete contract"
        );
    }

    "endpoint exception is not terminal-worthy for requested tier".to_string()
}

