//! Helper `endpoint_exception_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn endpoint_exception_summary(
    exceptions: &[EndpointExceptionRow],
    route: &str,
    tier: &str,
) -> String {
    let route_exceptions = endpoint_exceptions_for_route(exceptions, route, tier);
    if route_exceptions.is_empty() {
        return "-".to_string();
    }
    route_exceptions
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.endpoint_role.trim(),
                row.exception_type.trim(),
                row.evidence_level.trim()
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

