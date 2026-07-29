//! Helper `national_bundle_matches_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_bundle_matches_route(bundle: &NationalSegmentBundleRow, route: &str) -> bool {
    let key = canonical_route_key(route);
    bundle
        .route_labels
        .split(';')
        .chain(bundle.bundle_aliases.split(';').filter_map(|alias| {
            alias
                .strip_prefix("route:")
                .or_else(|| alias.strip_prefix("route-label:"))
        }))
        .any(|candidate| canonical_route_key(candidate) == key)
}

