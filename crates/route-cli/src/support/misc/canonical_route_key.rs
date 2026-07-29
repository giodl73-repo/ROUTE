//! Helper `canonical_route_key`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn canonical_route_key(route: &str) -> String {
    route
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_uppercase())
        .collect()
}

