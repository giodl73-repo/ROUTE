/// T1 network connectivity analysis.
///
/// Tests whether all T1 corridor endpoints can reach each other
/// using only T1 corridors — no T2, T3, or T4 allowed.
///
/// A T1 network gap exists when two T1 endpoints can only connect
/// via a T2 or lower corridor. This is a structural deficiency —
/// the national trunk-line system is not internally connected.
use crate::graph::HighwayGraph;
use crate::tier::T1_BACKBONE_ROUTES;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

/// An endpoint of a T1 corridor.
#[derive(Debug, Clone)]
pub struct T1Endpoint {
    pub route_id: String,
    pub node: NodeIndex,
    /// Approximate location (lon, lat)
    pub lon: f64,
    pub lat: f64,
}

/// Result of a T1 connectivity test between two endpoints.
#[derive(Debug, Clone)]
pub struct ConnectivityResult {
    pub from_route: String,
    pub to_route: String,
    /// Shortest path length staying on T1 only (None if unreachable on T1)
    pub t1_only_miles: Option<f64>,
    /// Shortest path length using all corridors
    pub all_corridors_miles: Option<f64>,
    /// Detour factor: t1_only / all_corridors (1.0 = no detour; >1.0 = T1 forces a detour)
    pub detour_factor: Option<f64>,
    /// True if T1-only path is impossible (requires T2 to connect)
    pub requires_t2: bool,
    /// Estimated missing link: the T2 corridor being used as a bridge
    pub t2_bridge: Option<String>,
}

/// Full connectivity report for the T1 network.
#[derive(Debug)]
pub struct T1ConnectivityReport {
    pub endpoints: Vec<T1Endpoint>,
    /// Results for each pair of T1 endpoints
    pub pair_results: Vec<ConnectivityResult>,
    /// Is the T1 network fully connected (all endpoints reachable on T1 only)?
    pub is_fully_connected: bool,
    /// Endpoint pairs that require T2 bridges
    pub gaps: Vec<ConnectivityResult>,
    /// Summary: which T1 corridors have isolated endpoints
    pub isolated_terminals: Vec<String>,
}

/// Run the T1 connectivity analysis.
pub fn analyze_t1_connectivity(g: &HighwayGraph) -> T1ConnectivityReport {
    // Find terminus nodes for each T1 corridor (westernmost and easternmost/southernmost)
    let endpoints = find_t1_endpoints(g);

    // Build T1-only subgraph edge set
    let t1_edge_set: HashSet<_> = g
        .graph
        .edge_indices()
        .filter(|&ei| T1_BACKBONE_ROUTES.contains(&g.graph[ei].route_id.as_str()))
        .collect();

    let mut pair_results = Vec::new();
    let mut any_gap = false;
    let mut isolated: HashSet<String> = HashSet::new();

    // Test all pairs of T1 endpoints
    for i in 0..endpoints.len() {
        for j in (i + 1)..endpoints.len() {
            if endpoints[i].route_id == endpoints[j].route_id {
                continue;
            } // same corridor

            let from = &endpoints[i];
            let to = &endpoints[j];

            // T1-only path
            let t1_dist = dijkstra_filtered(g, from.node, to.node, |ei| t1_edge_set.contains(&ei));

            // All-corridor path
            let all_dist = dijkstra_filtered(g, from.node, to.node, |_| true);

            let requires_t2 = t1_dist.is_none() && all_dist.is_some();
            let detour = t1_dist
                .zip(all_dist)
                .map(|(t1, all)| if all > 0.0 { t1 / all } else { 1.0 });

            if requires_t2 {
                any_gap = true;
                isolated.insert(from.route_id.clone());
                isolated.insert(to.route_id.clone());
            }

            pair_results.push(ConnectivityResult {
                from_route: from.route_id.clone(),
                to_route: to.route_id.clone(),
                t1_only_miles: t1_dist,
                all_corridors_miles: all_dist,
                detour_factor: detour,
                requires_t2,
                t2_bridge: None, // TODO: identify the bridging T2 corridor
            });
        }
    }

    let gaps: Vec<_> = pair_results
        .iter()
        .filter(|r| r.requires_t2 || r.detour_factor.map(|d| d > 1.5).unwrap_or(false))
        .cloned()
        .collect();

    T1ConnectivityReport {
        endpoints,
        is_fully_connected: !any_gap,
        gaps,
        isolated_terminals: isolated.into_iter().collect(),
        pair_results,
    }
}

/// Find terminus nodes for each T1 corridor.
/// Uses four extreme points (min/max lon, min/max lat) to find both termini
/// for both E-W (I-80, I-10) and N-S (I-5, I-95) corridors.
fn find_t1_endpoints(g: &HighwayGraph) -> Vec<T1Endpoint> {
    let mut endpoints = Vec::new();

    for &route_id in T1_BACKBONE_ROUTES {
        let edges = g.route_edges(route_id);
        if edges.is_empty() {
            continue;
        }

        // Collect all valid CONUS nodes for this corridor
        let mut all_nodes: Vec<(NodeIndex, f64, f64)> = Vec::new();
        for &ei in edges {
            if let Some((s, t)) = g.graph.edge_endpoints(ei) {
                for ni in [s, t] {
                    let c = &g.graph[ni].coord;
                    if c.x > -125.0 && c.x < -66.0 && c.y > 24.0 && c.y < 50.0 {
                        all_nodes.push((ni, c.x, c.y));
                    }
                }
            }
        }
        if all_nodes.len() < 2 {
            continue;
        }

        // Find the two nodes that are farthest apart (true termini)
        // For large corridors, sample to avoid O(N²) cost
        let sample: Vec<_> = if all_nodes.len() > 200 {
            all_nodes.iter().step_by(all_nodes.len() / 100).collect()
        } else {
            all_nodes.iter().collect()
        };

        let mut max_dist = 0.0f64;
        let mut term_a = all_nodes[0];
        let mut term_b = all_nodes[all_nodes.len() - 1];

        for (i, a) in sample.iter().enumerate() {
            for b in sample.iter().skip(i + 1) {
                let dx = a.1 - b.1;
                let dy = a.2 - b.2;
                let d = dx * dx + dy * dy;
                if d > max_dist {
                    max_dist = d;
                    term_a = (a.0, a.1, a.2);
                    term_b = (b.0, b.1, b.2);
                }
            }
        }

        for (node, lon, lat) in [term_a, term_b] {
            endpoints.push(T1Endpoint {
                route_id: route_id.to_string(),
                node,
                lon,
                lat,
            });
        }
    }

    endpoints
}

/// Dijkstra with an edge filter — only use edges where filter returns true.
fn dijkstra_filtered<F>(
    g: &HighwayGraph,
    source: NodeIndex,
    target: NodeIndex,
    edge_filter: F,
) -> Option<f64>
where
    F: Fn(petgraph::graph::EdgeIndex) -> bool,
{
    use ordered_float::NotNan;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist: HashMap<NodeIndex, f64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(NotNan<f64>, NodeIndex)>> = BinaryHeap::new();

    dist.insert(source, 0.0);
    heap.push(Reverse((NotNan::new(0.0).unwrap(), source)));

    while let Some(Reverse((cost, u))) = heap.pop() {
        let cost = cost.into_inner();
        if u == target {
            return Some(cost);
        }
        if cost > *dist.get(&u).unwrap_or(&f64::MAX) + 1e-9 {
            continue;
        }

        use petgraph::visit::EdgeRef;
        // Use BOTH directions of each edge (highways are bidirectional).
        // The directed graph stores edges in one direction; we traverse both.
        let both_dirs = g
            .graph
            .edges(u)
            .map(|er| (er.target(), er.weight().length_miles, er.id()))
            .chain(
                g.graph
                    .edges_directed(u, petgraph::Direction::Incoming)
                    .map(|er| (er.source(), er.weight().length_miles, er.id())),
            );
        for (v, edge_miles, eid) in both_dirs {
            if !edge_filter(eid) {
                continue;
            }
            if !edge_miles.is_finite() || edge_miles < 0.0 {
                continue;
            }
            let new_cost = cost + edge_miles;
            if new_cost < *dist.get(&v).unwrap_or(&f64::MAX) {
                dist.insert(v, new_cost);
                if let Ok(cost) = NotNan::new(new_cost) {
                    heap.push(Reverse((cost, v)));
                }
            }
        }
    }

    None
}
