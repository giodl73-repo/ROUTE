//! Helper `join_intermodal_to_corridor`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_intermodal_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    terminals: &[(f64, f64)],
) {
    if terminals.is_empty() {
        return;
    }
    let corridor_nodes: Vec<(f64, f64)> = graph
        .graph
        .node_indices()
        .filter(|&ni| {
            graph
                .graph
                .edges(ni)
                .any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| {
            let c = graph.graph[ni].coord;
            (c.x, c.y)
        })
        .collect();
    if corridor_nodes.is_empty() {
        return;
    }

    fn haversine2(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 3_958.8_f64;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        r * 2.0 * a.sqrt().asin()
    }

    let count = terminals
        .iter()
        .filter(|&&(tlat, tlon)| {
            corridor_nodes
                .iter()
                .any(|&(nx, ny)| haversine2(ny, nx, tlat, tlon) <= 30.0)
        })
        .count();
    attrs.intermodal_hub_count = count.min(255) as u8;
}
