//! Helper `t1_sla_pairs_by_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_sla_pairs_by_route(
    sla_rows: &[T1SlaPairRow],
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut pairs_by_route = std::collections::BTreeMap::<String, Vec<String>>::new();
    for pair in sla_rows {
        for route in pair.required_routes.split(';') {
            let route_key = canonical_route_key(route);
            if !route_key.is_empty() {
                pairs_by_route
                    .entry(route_key)
                    .or_default()
                    .push(pair.pair_id.clone());
            }
        }
    }
    for pairs in pairs_by_route.values_mut() {
        pairs.sort();
        pairs.dedup();
    }
    pairs_by_route
}

