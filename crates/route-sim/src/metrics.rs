/// Simulation output metrics.
///
/// All metrics are computed from a FlowState on a HighwayGraph.
/// The same metrics are used for baseline, post-incident, and post-intervention.
use crate::assignment::{bpr_travel_time, edge_capacity_vph, free_flow_time_hours, FlowState};
use petgraph::graph::EdgeIndex;
use route_network::HighwayGraph;
use std::collections::HashMap;

/// Full set of simulation output metrics for a corridor or network.
#[derive(Debug, Clone)]
pub struct SimMetrics {
    /// Total vehicles per hour through the network
    pub total_throughput_vph: f64,
    /// Total truck-hours of travel (sum of truck_flow × travel_time per edge)
    pub total_truck_hours: f64,
    /// Volume-weighted mean PTI across all edges
    pub mean_pti: f64,
    /// 90th-percentile PTI (worst-decile edges, weighted by truck flow)
    pub p90_pti: f64,
    /// Mean V/C ratio across all edges
    pub mean_vc: f64,
    /// Count of edges at LOS F (V/C > 1.0)
    pub losf_edges: usize,
    /// Estimated freight cost per hour ($M/hr)
    /// Uses ATRI cost of trucking: ~$2.50/hr per truck delay minute → $150/hr per truck
    pub freight_cost_per_hour_m: f64,
}

/// PTI (Planning Time Index) for a single edge:
/// 95th-percentile travel time / free-flow travel time.
/// Estimated from BPR at V/C = 0.95 (typical 95th-pct volume).
pub fn edge_pti(
    g: &HighwayGraph,
    ei: EdgeIndex,
    flow_state: &FlowState,
    capacities: &HashMap<EdgeIndex, f64>,
) -> f64 {
    let ff = free_flow_time_hours(g, ei);
    if ff <= 0.0 {
        return 1.0;
    }
    let cap = capacities
        .get(&ei)
        .cloned()
        .unwrap_or(edge_capacity_vph(g, ei));
    // 95th-pct volume ≈ mean flow × 1.15 (typical peak hour factor)
    let v_p95 = flow_state.flow.get(&ei).cloned().unwrap_or(0.0) * 1.15;
    let t_p95 = bpr_travel_time(ff, v_p95, cap, &Default::default());
    t_p95 / ff
}

/// PTI for a named corridor (aggregate across all corridor edges, truck-flow weighted).
pub fn corridor_pti(
    g: &HighwayGraph,
    route_id: &str,
    flow_state: &FlowState,
    capacities: &HashMap<EdgeIndex, f64>,
) -> f64 {
    let edges = g.route_edges(route_id);
    if edges.is_empty() {
        return 1.0;
    }

    let mut weighted_pti = 0.0;
    let mut total_weight = 0.0;

    for &ei in edges {
        let pti = edge_pti(g, ei, flow_state, capacities);
        let truck_flow = flow_state.truck_flow.get(&ei).cloned().unwrap_or(0.0);
        let weight = truck_flow.max(0.1); // avoid zero-weight
        weighted_pti += pti * weight;
        total_weight += weight;
    }

    if total_weight > 0.0 {
        weighted_pti / total_weight
    } else {
        1.0
    }
}

/// Network throughput: total vehicles passing through the network per hour.
pub fn network_throughput(flow_state: &FlowState) -> f64 {
    flow_state.flow.values().sum::<f64>()
}

/// Freight cost delta between two flow states.
/// Uses ATRI estimate: $150/hour per delayed truck (delay = actual - free-flow travel time).
/// Returns cost difference in $M per peak hour.
pub fn freight_cost_delta(
    g: &HighwayGraph,
    baseline: &FlowState,
    scenario: &FlowState,
    capacities: &HashMap<EdgeIndex, f64>,
) -> f64 {
    const ATRI_COST_PER_TRUCK_HOUR: f64 = 150.0; // USD per truck per hour delay
    const TO_MILLIONS: f64 = 1.0 / 1_000_000.0;

    let edge_delay_cost = |state: &FlowState| -> f64 {
        g.graph
            .edge_indices()
            .map(|ei| {
                let truck_flow = state.truck_flow.get(&ei).cloned().unwrap_or(0.0);
                let ff = free_flow_time_hours(g, ei);
                let v = state.flow.get(&ei).cloned().unwrap_or(0.0);
                let c = capacities
                    .get(&ei)
                    .cloned()
                    .unwrap_or(edge_capacity_vph(g, ei));
                let actual_t = bpr_travel_time(ff, v, c, &Default::default());
                let delay_hours = (actual_t - ff).max(0.0);
                truck_flow * delay_hours * ATRI_COST_PER_TRUCK_HOUR
            })
            .sum()
    };

    (edge_delay_cost(scenario) - edge_delay_cost(baseline)) * TO_MILLIONS
}

/// Compute full SimMetrics from a flow state.
pub fn compute_metrics(
    g: &HighwayGraph,
    flow_state: &FlowState,
    capacities: &HashMap<EdgeIndex, f64>,
) -> SimMetrics {
    let mut pti_vals: Vec<(f64, f64)> = Vec::new(); // (pti, truck_flow)
    let mut vc_sum = 0.0f64;
    let mut edge_count = 0usize;
    let mut losf = 0usize;
    let mut truck_hours = 0.0f64;

    for ei in g.graph.edge_indices() {
        let v = flow_state.flow.get(&ei).cloned().unwrap_or(0.0);
        let truck_v = flow_state.truck_flow.get(&ei).cloned().unwrap_or(0.0);
        let c = capacities
            .get(&ei)
            .cloned()
            .unwrap_or(edge_capacity_vph(g, ei));
        let ff = free_flow_time_hours(g, ei);
        let t = bpr_travel_time(ff, v, c, &Default::default());

        let vc = if c > 0.0 { v / c } else { 0.0 };
        let pti = edge_pti(g, ei, flow_state, capacities);

        pti_vals.push((pti, truck_v.max(0.1)));
        vc_sum += vc;
        edge_count += 1;
        if vc > 1.0 {
            losf += 1;
        }
        truck_hours += truck_v * t;
    }

    // Truck-flow-weighted mean PTI
    let total_weight: f64 = pti_vals.iter().map(|(_, w)| w).sum();
    let mean_pti = pti_vals.iter().map(|(p, w)| p * w).sum::<f64>() / total_weight.max(1.0);

    // P90 PTI — 90th percentile by truck flow weight
    pti_vals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let p90_target = total_weight * 0.90;
    let mut cumulative = 0.0;
    let mut p90_pti = 1.0;
    for (p, w) in &pti_vals {
        cumulative += w;
        if cumulative >= p90_target {
            p90_pti = *p;
            break;
        }
    }

    // Set baseline flows to 0 for free-flow comparison.
    let zero_flow = FlowState::empty();
    let freight_cost = freight_cost_delta(g, &zero_flow, flow_state, capacities);

    SimMetrics {
        total_throughput_vph: network_throughput(flow_state),
        total_truck_hours: truck_hours,
        mean_pti,
        p90_pti,
        mean_vc: if edge_count > 0 {
            vc_sum / edge_count as f64
        } else {
            0.0
        },
        losf_edges: losf,
        freight_cost_per_hour_m: freight_cost,
    }
}
