//! Helper `endpoint_exceptions_for_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn endpoint_exceptions_for_route<'a>(
    exceptions: &'a [EndpointExceptionRow],
    route: &str,
    tier: &str,
) -> Vec<&'a EndpointExceptionRow> {
    let route = normalise_designation(route);
    exceptions
        .iter()
        .filter(|row| normalise_designation(&row.route) == route)
        .filter(|row| row.requested_tier.trim().eq_ignore_ascii_case(tier.trim()))
        .collect()
}

