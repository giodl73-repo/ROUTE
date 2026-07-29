//! Helper `atlas_candidate_ids`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn atlas_candidate_ids(graph: &route_network::HighwayGraph) -> Vec<String> {
    let mut ids = graph.interstate_ids();
    ids.extend(graph.us_highway_ids());
    ids.sort();
    ids.dedup();
    ids
}

