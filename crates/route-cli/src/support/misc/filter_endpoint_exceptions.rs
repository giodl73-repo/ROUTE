//! Helper `filter_endpoint_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn filter_endpoint_exceptions<'a>(
    rows: &'a [EndpointExceptionRow],
    tier: Option<&str>,
    route: Option<&str>,
) -> Vec<&'a EndpointExceptionRow> {
    let route = route.map(normalise_designation);
    rows.iter()
        .filter(|row| {
            tier.map(|tier| row.requested_tier.trim().eq_ignore_ascii_case(tier.trim()))
                .unwrap_or(true)
        })
        .filter(|row| {
            route
                .as_ref()
                .map(|route| normalise_designation(&row.route) == *route)
                .unwrap_or(true)
        })
        .collect()
}
