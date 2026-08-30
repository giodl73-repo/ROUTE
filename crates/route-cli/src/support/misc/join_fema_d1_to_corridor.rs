//! Helper `join_fema_d1_to_corridor`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_fema_d1_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    tiles: &[FemaTile],
) {
    if tiles.is_empty() {
        return;
    }

    let edge_boxes: Vec<(f64, f64, f64, f64)> = graph
        .route_edges(route_id)
        .iter()
        .filter_map(|&ei| {
            let edge = &graph.graph[ei];
            let mut coords = edge.geometry.points().map(|p| (p.x(), p.y()));
            let first = coords.next()?;
            let (mut xmin, mut ymin, mut xmax, mut ymax) = (first.0, first.1, first.0, first.1);
            for (x, y) in coords {
                xmin = xmin.min(x);
                xmax = xmax.max(x);
                ymin = ymin.min(y);
                ymax = ymax.max(y);
            }
            Some((xmin, ymin, xmax, ymax))
        })
        .collect();
    if edge_boxes.is_empty() {
        return;
    }

    let total_sfha: u64 = tiles
        .iter()
        .filter(|t| {
            if route_id == "I80" && !t.name.starts_with("I80-") {
                return false;
            }
            if t.status != "ok" {
                return false;
            }
            edge_boxes.iter().any(|&(xmin, ymin, xmax, ymax)| {
                !(xmax < t.xmin || xmin > t.xmax || ymax < t.ymin || ymin > t.ymax)
            })
        })
        .map(|t| t.sfha_count as u64)
        .sum();

    if total_sfha == 0 {
        return;
    }

    // Avg SFHA polygon spans ~0.3 miles → convert feature count to miles
    let sfha_miles = total_sfha as f64 * 0.3;
    attrs.fema_sfha_miles = Some(sfha_miles);
    // Proxy: 70% of total is consecutive for coastal/valley corridors
    attrs.max_consecutive_sfha_miles = Some((sfha_miles * 0.7) as f32);
}
