//! Helper `tier_pavement_route_state_scope`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_route_state_scope(
    graph: Option<&route_network::HighwayGraph>,
    route: &str,
) -> String {
    graph
        .and_then(|graph| route_network::aggregate_corridor(graph, route))
        .map(|corridor| corridor.states.join(";"))
        .unwrap_or_default()
}
