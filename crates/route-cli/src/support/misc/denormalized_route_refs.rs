//! Helper `denormalized_route_refs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn denormalized_route_refs(routes: &str) -> String {
    routes
        .split([';', ','])
        .map(str::trim)
        .filter(|route| !route.is_empty())
        .map(|route| {
            let norm = normalise_designation(route);
            if let Some(rest) = norm.strip_prefix('I') {
                format!("I-{rest}")
            } else if let Some(rest) = norm.strip_prefix("US") {
                format!("US{rest}")
            } else {
                norm
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}
