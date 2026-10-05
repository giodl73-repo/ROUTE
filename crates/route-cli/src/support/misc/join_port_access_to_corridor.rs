//! Helper `join_port_access_to_corridor`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_port_access_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    ports: &[PortLocation],
) {
    if ports.is_empty() {
        return;
    }

    // Get terminus nodes (degree-1 interchange nodes on this route)
    let node_coords: Vec<(f64, f64)> = graph
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
    if node_coords.is_empty() {
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

    let mut min_dist = f64::MAX;
    let mut terminus_flag = false;
    let mut border_flag = false;

    for port in ports {
        for &(px, py) in &node_coords {
            let d = haversine(py, px, port.lat, port.lon);
            if d < min_dist {
                min_dist = d;
            }
            if d <= 30.0 {
                if port.is_border {
                    border_flag = true;
                } else {
                    terminus_flag = true;
                }
            }
        }
    }

    attrs.port_terminus_flag = terminus_flag;
    attrs.border_crossing_flag = border_flag;
    if min_dist < f64::MAX {
        attrs.nearest_top25_port_miles = Some(min_dist as f32);
    }
}
