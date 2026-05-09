/// Wardrop User Equilibrium traffic assignment via Frank-Wolfe algorithm.
///
/// Wardrop's first principle: at equilibrium, no traveler can reduce their
/// travel time by unilaterally switching routes. This is the correct model
/// for highway networks where users route selfishly.
///
/// Frank-Wolfe iterative algorithm:
///   1. Initialize: assign all demand via free-flow shortest paths (all-or-nothing)
///   2. Compute BPR travel times from current flows
///   3. Find descent direction: all-or-nothing assignment using current travel times
///   4. Line search: find optimal step size λ ∈ [0,1]
///   5. Update flows: x_{n+1} = (1-λ)x_n + λ·direction
///   6. Check convergence (relative gap < tolerance); if not, go to 2
///
/// BPR (Bureau of Public Roads) travel time function:
///   t(v) = t_0 × (1 + α × (v/c)^β)
///   Standard parameters: α=0.15, β=4.0
use crate::demand::{DemandMatrix, OdDemand};
use petgraph::algo::dijkstra;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef; // needed for .id(), .source(), .target() on EdgeReference
use route_network::HighwayGraph;
use std::collections::HashMap;

/// BPR (Bureau of Public Roads) travel time function parameters.
#[derive(Debug, Clone)]
pub struct BprParams {
    /// Typically 0.15
    pub alpha: f64,
    /// Typically 4.0
    pub beta: f64,
}

impl Default for BprParams {
    fn default() -> Self {
        BprParams {
            alpha: 0.15,
            beta: 4.0,
        }
    }
}

/// Current flow state: vehicles per hour on each edge.
#[derive(Debug, Clone)]
pub struct FlowState {
    /// Total flow (vph) on each edge
    pub flow: HashMap<EdgeIndex, f64>,
    /// Truck flow separately tracked
    pub truck_flow: HashMap<EdgeIndex, f64>,
    /// Current BPR travel time on each edge (hours per mile → total hours)
    pub travel_time: HashMap<EdgeIndex, f64>,
    /// Number of Frank-Wolfe iterations run
    pub iterations: usize,
    /// Relative gap at convergence
    pub relative_gap: f64,
}

impl FlowState {
    pub fn empty() -> Self {
        FlowState {
            flow: HashMap::new(),
            truck_flow: HashMap::new(),
            travel_time: HashMap::new(),
            iterations: 0,
            relative_gap: f64::MAX,
        }
    }

    /// Volume-to-capacity ratio for an edge.
    pub fn vc_ratio(&self, ei: EdgeIndex, capacity_vph: f64) -> f64 {
        let v = self.flow.get(&ei).cloned().unwrap_or(0.0);
        if capacity_vph > 0.0 {
            v / capacity_vph
        } else {
            0.0
        }
    }
}

/// Edge capacity in vehicles per hour.
/// Standard: lane_count × 1,900 pcph (HCM LOS E service flow rate)
pub fn edge_capacity_vph(g: &HighwayGraph, ei: EdgeIndex) -> f64 {
    let edge = &g.graph[ei];
    let lanes = edge.lane_count.unwrap_or(4) as f64; // default 4 lanes (2+2)
    lanes / 2.0 * 1_900.0 // lanes per direction × peak capacity per lane
}

/// BPR travel time in hours for an edge given current flow.
pub fn bpr_travel_time(
    free_flow_hours: f64,
    flow_vph: f64,
    capacity_vph: f64,
    params: &BprParams,
) -> f64 {
    if capacity_vph <= 0.0 {
        return free_flow_hours * 10.0;
    } // blocked
    free_flow_hours * (1.0 + params.alpha * (flow_vph / capacity_vph).powf(params.beta))
}

/// Free-flow travel time in hours: length_miles / speed_mph.
pub fn free_flow_time_hours(g: &HighwayGraph, ei: EdgeIndex) -> f64 {
    let edge = &g.graph[ei];
    let speed = edge.speed_limit.unwrap_or(65) as f64;
    edge.length_miles / speed
}

/// Run Wardrop User Equilibrium via Frank-Wolfe.
///
/// Convergence: relative gap < tolerance OR max_iterations reached.
/// Relative gap = (current objective - lower bound) / current objective
pub fn wardrop_equilibrium(
    g: &HighwayGraph,
    demand: &DemandMatrix,
    capacities: &HashMap<EdgeIndex, f64>,
    bpr: &BprParams,
    max_iterations: usize,
    tolerance: f64,
) -> FlowState {
    let mut state = FlowState::empty();

    // Initialize: compute free-flow travel times
    for ei in g.graph.edge_indices() {
        let ff = free_flow_time_hours(g, ei);
        state.travel_time.insert(ei, ff);
        state.flow.insert(ei, 0.0);
        state.truck_flow.insert(ei, 0.0);
    }

    // Step 1: Initial all-or-nothing assignment (free-flow)
    let aon = all_or_nothing(g, demand, &state.travel_time);
    for (ei, v) in &aon.flow {
        *state.flow.entry(*ei).or_insert(0.0) += v;
    }
    for (ei, v) in &aon.truck_flow {
        *state.truck_flow.entry(*ei).or_insert(0.0) += v;
    }

    for iter in 0..max_iterations {
        // Update BPR travel times from current flows
        for ei in g.graph.edge_indices() {
            let v = state.flow.get(&ei).cloned().unwrap_or(0.0);
            let c = capacities.get(&ei).cloned().unwrap_or(1900.0);
            let ff = free_flow_time_hours(g, ei);
            let t = bpr_travel_time(ff, v, c, bpr);
            state.travel_time.insert(ei, t);
        }

        // All-or-nothing with current travel times (descent direction)
        let direction = all_or_nothing(g, demand, &state.travel_time);

        // Compute relative gap
        let current_obj: f64 = g
            .graph
            .edge_indices()
            .map(|ei| {
                let v = state.flow.get(&ei).cloned().unwrap_or(0.0);
                let t = state.travel_time.get(&ei).cloned().unwrap_or(0.0);
                v * t
            })
            .sum();
        let aon_obj: f64 = g
            .graph
            .edge_indices()
            .map(|ei| {
                let v = direction.flow.get(&ei).cloned().unwrap_or(0.0);
                let t = state.travel_time.get(&ei).cloned().unwrap_or(0.0);
                v * t
            })
            .sum();
        let gap = if current_obj > 0.0 {
            (current_obj - aon_obj).abs() / current_obj
        } else {
            0.0
        };

        state.relative_gap = gap;
        state.iterations = iter + 1;

        if gap < tolerance {
            break;
        }

        // Line search: bisection to find optimal λ
        let lambda = line_search(g, &state, &direction, capacities, bpr, demand);

        // Update flows: x = (1-λ)x + λ·direction
        for ei in g.graph.edge_indices() {
            let x = state.flow.get(&ei).cloned().unwrap_or(0.0);
            let d = direction.flow.get(&ei).cloned().unwrap_or(0.0);
            state.flow.insert(ei, (1.0 - lambda) * x + lambda * d);

            let xt = state.truck_flow.get(&ei).cloned().unwrap_or(0.0);
            let dt = direction.truck_flow.get(&ei).cloned().unwrap_or(0.0);
            state
                .truck_flow
                .insert(ei, (1.0 - lambda) * xt + lambda * dt);
        }
    }

    state
}

/// All-or-nothing assignment: send all demand on the single shortest path.
/// Returns edge flows (not accumulated — this is the direction vector only).
fn all_or_nothing(
    g: &HighwayGraph,
    demand: &DemandMatrix,
    travel_times: &HashMap<EdgeIndex, f64>,
) -> FlowState {
    let mut state = FlowState::empty();

    for od in demand {
        // Dijkstra from origin using current travel times as weights
        let path_flows = shortest_path_flow(g, od.origin, od.destination, travel_times, od);
        for (ei, v) in path_flows.flow {
            *state.flow.entry(ei).or_insert(0.0) += v;
        }
        for (ei, v) in path_flows.truck_flow {
            *state.truck_flow.entry(ei).or_insert(0.0) += v;
        }
    }

    state
}

/// Find the shortest path and load demand onto it.
fn shortest_path_flow(
    g: &HighwayGraph,
    origin: NodeIndex,
    destination: NodeIndex,
    travel_times: &HashMap<EdgeIndex, f64>,
    od: &OdDemand,
) -> FlowState {
    let mut result = FlowState::empty();

    // Edge weight function: travel time
    let dist = dijkstra(&g.graph, origin, Some(destination), |er| {
        travel_times
            .get(&er.id())
            .cloned()
            .unwrap_or(free_flow_time_hours(g, er.id()))
    });

    if !dist.contains_key(&destination) {
        return result; // no path found
    }

    // Reconstruct path by tracing back predecessors
    // Simplified: identify edges on any path segment via flow conservation
    // For production use, implement proper predecessor tracking
    // Here we use a heuristic: edges whose endpoints are on the shortest-path tree
    let path_edges = find_path_edges(g, origin, destination, &dist, travel_times);
    let total_demand = od.truck_vph + od.car_vph;
    let truck_fraction = if total_demand > 0.0 {
        od.truck_vph / total_demand
    } else {
        0.0
    };

    for ei in path_edges {
        *result.flow.entry(ei).or_insert(0.0) += total_demand;
        *result.truck_flow.entry(ei).or_insert(0.0) += total_demand * truck_fraction;
    }

    result
}

/// Find edges on the shortest path from origin to destination.
/// Uses the distance labels from Dijkstra to identify path edges.
fn find_path_edges(
    g: &HighwayGraph,
    origin: NodeIndex,
    destination: NodeIndex,
    dist: &HashMap<NodeIndex, f64>,
    travel_times: &HashMap<EdgeIndex, f64>,
) -> Vec<EdgeIndex> {
    let mut path = Vec::new();
    let mut current = destination;
    let mut visited = std::collections::HashSet::new();

    while current != origin {
        if visited.contains(&current) {
            break;
        } // cycle guard
        visited.insert(current);

        // Find the incoming edge that is on the shortest path
        let best = g
            .graph
            .edges_directed(current, petgraph::Direction::Incoming)
            .filter_map(|er| {
                let pred = er.source();
                let t = travel_times
                    .get(&er.id())
                    .cloned()
                    .unwrap_or(free_flow_time_hours(g, er.id()));
                let pred_dist = dist.get(&pred).cloned()?;
                let curr_dist = dist.get(&current).cloned()?;
                // Edge is on shortest path if pred_dist + edge_time ≈ curr_dist
                if (pred_dist + t - curr_dist).abs() < 1e-9 {
                    Some((er.id(), pred))
                } else {
                    None
                }
            })
            .next();

        match best {
            Some((ei, pred)) => {
                path.push(ei);
                current = pred;
            }
            None => break,
        }
    }

    path.reverse();
    path
}

/// Line search: find λ ∈ [0,1] that minimizes the BPR objective.
/// Uses bisection (simple but robust for convex BPR).
fn line_search(
    g: &HighwayGraph,
    current: &FlowState,
    direction: &FlowState,
    capacities: &HashMap<EdgeIndex, f64>,
    bpr: &BprParams,
    _demand: &DemandMatrix,
) -> f64 {
    let objective = |lambda: f64| -> f64 {
        g.graph
            .edge_indices()
            .map(|ei| {
                let x = current.flow.get(&ei).cloned().unwrap_or(0.0);
                let d = direction.flow.get(&ei).cloned().unwrap_or(0.0);
                let v = (1.0 - lambda) * x + lambda * d;
                let c = capacities.get(&ei).cloned().unwrap_or(1900.0);
                let ff = free_flow_time_hours(g, ei);
                // BPR integral: t_0 × [v + α × v^(β+1) / ((β+1) × c^β)]
                ff * (v + bpr.alpha * v.powf(bpr.beta + 1.0)
                    / ((bpr.beta + 1.0) * c.powf(bpr.beta)))
            })
            .sum()
    };

    // Bisection on derivative of objective w.r.t. lambda
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for _ in 0..20 {
        let mid = (lo + hi) / 2.0;
        let f_lo = objective(lo + 1e-6) - objective(lo);
        let f_mid = objective(mid + 1e-6) - objective(mid);
        if f_lo * f_mid < 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (lo + hi) / 2.0
}
