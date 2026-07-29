//! Helper `bridge_standard_missing_routes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn bridge_standard_missing_routes(
    routes: &[String],
    nbi: &std::collections::HashMap<String, NbiBridgeRecord>,
) -> Vec<String> {
    routes
        .iter()
        .filter(|route| !nbi.contains_key(*route))
        .cloned()
        .collect()
}

