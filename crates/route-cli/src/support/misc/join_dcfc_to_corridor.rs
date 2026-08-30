//! Helper `join_dcfc_to_corridor`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_dcfc_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    corridor_miles: f64,
    attrs: &mut route_network::CorridorAttributes,
    dcfc_stations: &[(f64, f64)],
) {
    if dcfc_stations.is_empty() {
        return;
    }

    // Get all nodes on this corridor
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

    fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 3_958.8_f64;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        r * 2.0 * a.sqrt().asin()
    }

    // Count DCFC stations within 5 miles of any corridor node
    let mut count = 0u32;
    for &(slat, slon) in dcfc_stations {
        let near = corridor_nodes
            .iter()
            .any(|&(nx, ny)| haversine(ny, nx, slat, slon) <= 5.0);
        if near {
            count += 1;
        }
    }

    if corridor_miles > 0.0 {
        let dcfc_per_100 = (count as f64 / corridor_miles) * 100.0;
        attrs.dcfc_per_100mi = Some(dcfc_per_100 as f32);
    }
}
