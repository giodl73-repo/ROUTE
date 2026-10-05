//! Helper `scenario_edge_candidates`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn scenario_edge_candidates(
    graph: &route_network::HighwayGraph,
    route: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
    top: usize,
) -> Vec<ScenarioEdgeCandidate> {
    let mut candidates: Vec<ScenarioEdgeCandidate> = graph
        .route_edges(route)
        .iter()
        .filter_map(|&ei| {
            let edge = &graph.graph[ei];
            let (mid_lat, mid_lon) = edge_midpoint(edge)?;
            let distance_miles = haversine_miles(lat, lon, mid_lat, mid_lon);
            (distance_miles <= radius_miles).then(|| ScenarioEdgeCandidate {
                edge_id: edge.id,
                distance_miles,
                length_miles: edge.length_miles,
                aadt: edge.aadt,
                lanes: edge.lane_count,
                state: edge.state.clone(),
                mid_lat,
                mid_lon,
            })
        })
        .collect();

    candidates.sort_by(|a, b| {
        a.distance_miles
            .partial_cmp(&b.distance_miles)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.edge_id.cmp(&b.edge_id))
    });
    candidates.truncate(top);
    candidates
}
