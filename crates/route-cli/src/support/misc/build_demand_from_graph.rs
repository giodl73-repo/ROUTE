//! Helper `build_demand_from_graph`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn build_demand_from_graph(
    g: &route_network::HighwayGraph,
) -> route_sim::demand::DemandMatrix {
    use route_sim::demand::{demand_from_aadt, DemandParams};
    let params = DemandParams::default();
    let mut demand = Vec::new();

    // Create O-D pairs from terminus nodes of each interstate
    for route_id in g.interstate_ids() {
        let edges = g.route_edges(&route_id);
        if edges.len() < 2 {
            continue;
        }

        // Use first and last edge endpoints as a crude O-D pair
        if let (Some(&first_ei), Some(&last_ei)) = (edges.first(), edges.last()) {
            if let (Some((s, _)), Some((_, t))) = (
                g.graph.edge_endpoints(first_ei),
                g.graph.edge_endpoints(last_ei),
            ) {
                let mean_aadt = edges
                    .iter()
                    .filter_map(|&ei| g.graph[ei].aadt.map(|a| a as f64))
                    .sum::<f64>()
                    / edges.len() as f64;
                let mean_pct = edges
                    .iter()
                    .filter_map(|&ei| g.graph[ei].pct_truck)
                    .sum::<f32>()
                    / edges.len() as f32;

                if mean_aadt > 0.0 {
                    demand.push(demand_from_aadt(mean_aadt, mean_pct, &params, s, t));
                }
            }
        }
    }
    demand
}
