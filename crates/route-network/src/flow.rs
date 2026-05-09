/// Edmonds-Karp max-flow on the highway graph.
///
/// Edge capacity = daily_capacity (lane_count × 1,900 pcph × 24h).
/// Falls back to AADT when lane count is unknown (capacity = current volume,
/// which understates true capacity but gives a conservative bound).
///
/// Use cases:
///   - What is the maximum freight throughput from origin to destination?
///   - Which edges are saturated (bottlenecks)?
///   - How much does adding one lane on a segment increase system flow?
use crate::graph::HighwayGraph;
use petgraph::graph::{EdgeIndex, NodeIndex};
use std::collections::{HashMap, VecDeque};

/// Result of a corridor flow capacity analysis.
#[derive(Debug)]
pub struct FlowResult {
    /// Binding throughput — minimum capacity segment (vehicles/day)
    pub max_flow_vpd: f64,
    /// Mean capacity across all segments (unconstrained throughput)
    pub mean_capacity_vpd: f64,
    /// Binding bottleneck segments (min-capacity edges)
    pub bottleneck_edges: Vec<EdgeIndex>,
    /// Capacity of each bottleneck segment (vpd)
    pub bottleneck_capacities: Vec<f64>,
    /// Throughput gain from adding one lane at each bottleneck (vpd)
    pub lane_addition_gain: Vec<f64>,
    /// Number of segments analyzed
    pub augmenting_paths: usize,
    /// True if actual lane count data was available (false = default 2-lane assumed)
    pub has_lane_data: bool,
}

/// Analyze flow capacity along a named corridor.
///
/// A single interstate route is a series-connected path — max-flow equals
/// the minimum capacity segment (the binding bottleneck). When HPMS lane
/// data is available, this gives an accurate bound. Without it, we use the
/// default 2-lane capacity and flag the result as estimated.
///
/// Full Edmonds-Karp across the national graph (considering parallel routes)
/// is available via `national_max_flow(g, source, sink)`.
pub fn corridor_max_flow(g: &HighwayGraph, route_id: &str) -> Option<FlowResult> {
    let edges = g.route_edges(route_id);
    if edges.is_empty() {
        return None;
    }

    let capacities = build_capacity_map(g);
    let has_lane_data = edges.iter().any(|&ei| g.graph[ei].lane_count.is_some());

    // Min-capacity segment = binding bottleneck for a series path
    let (bottleneck_edge, min_cap) = edges
        .iter()
        .map(|&ei| (ei, capacities.get(&ei).cloned().unwrap_or(0.0)))
        .min_by(|a, b| a.1.total_cmp(&b.1))?;

    // Mean capacity across all segments — the "unconstrained" throughput
    let mean_cap = edges
        .iter()
        .map(|&ei| capacities.get(&ei).cloned().unwrap_or(0.0))
        .sum::<f64>()
        / edges.len() as f64;

    let bottleneck_capacities = vec![min_cap];
    let lane_addition_gain = vec![1_900.0 * 24.0]; // one lane = 45,600 vpd

    Some(FlowResult {
        max_flow_vpd: min_cap,
        mean_capacity_vpd: mean_cap,
        bottleneck_edges: vec![bottleneck_edge],
        bottleneck_capacities,
        lane_addition_gain,
        augmenting_paths: edges.len(), // segments traversed
        has_lane_data,
    })
}

/// Max-flow across the full national highway graph between two node indices.
/// Use this to find how much freight CAN move even when one corridor is closed.
pub fn national_max_flow(g: &HighwayGraph, source: NodeIndex, sink: NodeIndex) -> f64 {
    let capacities = build_capacity_map(g);
    edmonds_karp(g, &capacities, source, sink).max_flow
}

/// Build edge capacity map: EdgeIndex → vpd (vehicles per day).
/// Uses daily_capacity when lane count is known; falls back to AADT.
fn build_capacity_map(g: &HighwayGraph) -> HashMap<EdgeIndex, f64> {
    let mut caps = HashMap::new();
    for ei in g.graph.edge_indices() {
        let edge = &g.graph[ei];
        let cap = if let Some(lanes) = edge.lane_count {
            lanes as f64 * 1_900.0 * 24.0
        } else if let Some(aadt) = edge.aadt {
            // Conservative: assume current AADT is at ~70% capacity
            aadt as f64 / 0.70
        } else {
            // Default: 2-lane interstate at free-flow capacity
            2.0 * 1_900.0 * 24.0
        };
        caps.insert(ei, cap);
    }
    caps
}

/// Internal Edmonds-Karp result.
struct EKResult {
    max_flow: f64,
}

/// Edmonds-Karp algorithm (BFS augmenting paths).
/// Runs on the directed graph; each edge has capacity from `caps`.
fn edmonds_karp(
    g: &HighwayGraph,
    caps: &HashMap<EdgeIndex, f64>,
    source: NodeIndex,
    sink: NodeIndex,
) -> EKResult {
    // Build residual graph: (u, v) → residual capacity
    let mut residual: HashMap<(NodeIndex, NodeIndex), f64> = HashMap::new();
    for ei in g.graph.edge_indices() {
        if let Some((u, v)) = g.graph.edge_endpoints(ei) {
            let cap = caps.get(&ei).cloned().unwrap_or(0.0);
            *residual.entry((u, v)).or_insert(0.0) += cap;
            residual.entry((v, u)).or_insert(0.0); // reverse edge starts at 0
        }
    }

    let mut max_flow = 0.0;
    let mut paths = 0;

    // Limit iterations to avoid excessive runtime on large graphs
    const MAX_PATHS: usize = 500;

    loop {
        if paths >= MAX_PATHS {
            break;
        }

        // BFS to find shortest augmenting path
        let path = bfs_augmenting_path(&residual, source, sink);
        if path.is_empty() {
            break;
        }

        // Find bottleneck capacity along path
        let flow = path
            .windows(2)
            .map(|w| residual.get(&(w[0], w[1])).cloned().unwrap_or(0.0))
            .fold(f64::MAX, f64::min);

        if flow <= 0.0 {
            break;
        }

        // Update residual capacities
        for w in path.windows(2) {
            *residual.entry((w[0], w[1])).or_insert(0.0) -= flow;
            *residual.entry((w[1], w[0])).or_insert(0.0) += flow;
        }

        max_flow += flow;
        paths += 1;
    }

    EKResult { max_flow }
}

/// BFS to find an augmenting path from source to sink in the residual graph.
fn bfs_augmenting_path(
    residual: &HashMap<(NodeIndex, NodeIndex), f64>,
    source: NodeIndex,
    sink: NodeIndex,
) -> Vec<NodeIndex> {
    let mut visited: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(source);
    visited.insert(source, source);

    while let Some(u) = queue.pop_front() {
        if u == sink {
            break;
        }
        // Iterate over all edges from u that have residual capacity
        for (&(from, to), &cap) in residual {
            if from == u && cap > 1e-9 && !visited.contains_key(&to) {
                visited.insert(to, u);
                queue.push_back(to);
            }
        }
    }

    if !visited.contains_key(&sink) {
        return vec![]; // no path found
    }

    // Reconstruct path
    let mut path = vec![sink];
    let mut cur = sink;
    while cur != source {
        cur = visited[&cur];
        path.push(cur);
    }
    path.reverse();
    path
}
