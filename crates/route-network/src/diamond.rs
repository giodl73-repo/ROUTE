/// Diamond intersection analysis.
///
/// For a named T1/T1 intersection, computes:
///   1. Current k-connectivity (min edge-disjoint paths between the two T1 corridors
///      within a 50-mile zone around the intersection)
///   2. What connector roads would bring k to ≥3
///   3. Estimated connector cost
///
/// The "50-mile diamond" concept: instead of a single interchange node,
/// create a distributed intersection zone with multiple independent cross-connections.
/// k=1 = single point of failure; target k≥3 = resilient diamond.
use crate::graph::HighwayGraph;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet, VecDeque};

const T1_ROUTES: &[&str] = &["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"];
const DIAMOND_RADIUS_DEG: f64 = 0.7; // ~50 miles at mid-latitude

/// A T1/T1 intersection point.
#[derive(Debug, Clone)]
pub struct T1Intersection {
    pub name: String,
    pub route_a: String,
    pub route_b: String,
    /// Center node of the intersection zone
    pub center: NodeIndex,
    pub lon: f64,
    pub lat: f64,
}

/// k-connectivity result for an intersection.
#[derive(Debug)]
pub struct DiamondResult {
    pub intersection: T1Intersection,
    /// Current k (min edge-disjoint paths between A and B within 50-mile zone)
    pub k_current: usize,
    /// Is this a single point of failure?
    pub is_spf: bool,
    /// How many additional connector roads needed to reach k=3?
    pub connectors_needed: usize,
    /// Estimated connector cost in $B (each connector ~$200-500M)
    pub est_cost_b: f64,
    /// Nodes in route A within the 50-mile zone
    pub zone_nodes_a: Vec<NodeIndex>,
    /// Nodes in route B within the 50-mile zone
    pub zone_nodes_b: Vec<NodeIndex>,
}

/// Find all T1/T1 intersections in the graph.
pub fn find_t1_intersections(g: &HighwayGraph) -> Vec<T1Intersection> {
    let mut intersections = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // Find nodes where two different T1 routes meet
    for ni in g.graph.node_indices() {
        let route_ids: Vec<String> = {
            let mut ids: Vec<_> = g
                .graph
                .edges(ni)
                .map(|er| er.weight().route_id.clone())
                .chain(
                    g.graph
                        .edges_directed(ni, petgraph::Direction::Incoming)
                        .map(|er| er.weight().route_id.clone()),
                )
                .filter(|id| T1_ROUTES.contains(&id.as_str()))
                .collect();
            ids.sort();
            ids.dedup();
            ids
        };

        if route_ids.len() < 2 {
            continue;
        }

        // Create an intersection for each unique T1 pair at this node
        for i in 0..route_ids.len() {
            for j in (i + 1)..route_ids.len() {
                let key = format!("{}x{}", route_ids[i], route_ids[j]);
                if seen.contains(&key) {
                    continue;
                }
                seen.insert(key);

                let c = &g.graph[ni].coord;
                let name = format!(
                    "{}/{}",
                    route_ids[i].replace('I', "I-"),
                    route_ids[j].replace('I', "I-")
                );

                intersections.push(T1Intersection {
                    name,
                    route_a: route_ids[i].clone(),
                    route_b: route_ids[j].clone(),
                    center: ni,
                    lon: c.x,
                    lat: c.y,
                });
            }
        }
    }

    intersections
}

/// Find T1 intersection by name or route pair (e.g. "I35xI80", "I-35xI-80").
pub fn find_intersection(g: &HighwayGraph, name: &str) -> Option<T1Intersection> {
    let norm = name.to_uppercase().replace('-', "").replace('X', "x");
    let all = find_t1_intersections(g);

    // Try exact match
    if let Some(ix) = all.iter().find(|ix| {
        format!("{}x{}", ix.route_a, ix.route_b) == norm
            || format!("{}x{}", ix.route_b, ix.route_a) == norm
    }) {
        return Some(ix.clone());
    }

    // Try partial match (any intersection containing both routes)
    let parts: Vec<&str> = norm.split('x').collect();
    if parts.len() == 2 {
        if let Some(ix) = all.iter().find(|ix| {
            (ix.route_a.contains(parts[0]) || ix.route_b.contains(parts[0]))
                && (ix.route_a.contains(parts[1]) || ix.route_b.contains(parts[1]))
        }) {
            return Some(ix.clone());
        }
    }

    None
}

/// Analyze diamond connectivity for a specific intersection.
pub fn analyze_diamond(g: &HighwayGraph, intersection: T1Intersection) -> DiamondResult {
    // Find all nodes of route A and route B within 50-mile zone of intersection center
    let cx = intersection.lon;
    let cy = intersection.lat;

    let zone_nodes_a = nodes_in_zone(g, &intersection.route_a, cx, cy, DIAMOND_RADIUS_DEG);
    let zone_nodes_b = nodes_in_zone(g, &intersection.route_b, cx, cy, DIAMOND_RADIUS_DEG);

    // Build subgraph of zone edges (all corridors within the zone)
    let zone_edges: HashSet<EdgeIndex> = g
        .graph
        .edge_indices()
        .filter(|&ei| {
            if let Some((s, t)) = g.graph.edge_endpoints(ei) {
                let cs = &g.graph[s].coord;
                let ct = &g.graph[t].coord;
                in_zone(cs.x, cs.y, cx, cy, DIAMOND_RADIUS_DEG * 1.5)
                    || in_zone(ct.x, ct.y, cx, cy, DIAMOND_RADIUS_DEG * 1.5)
            } else {
                false
            }
        })
        .collect();

    // k-connectivity = min edge-disjoint paths between any node in zone_a and any in zone_b
    // Simplified: count how many independent paths exist from zone_a to zone_b
    let k = compute_k_connectivity(g, &zone_nodes_a, &zone_nodes_b, &zone_edges);

    let is_spf = k <= 1;
    let connectors_needed = if k >= 3 { 0 } else { 3 - k };

    // Cost: each connector road ~10 miles at $25M/mile = $250M average
    let est_cost_b = connectors_needed as f64 * 0.25;

    DiamondResult {
        intersection,
        k_current: k,
        is_spf,
        connectors_needed,
        est_cost_b,
        zone_nodes_a,
        zone_nodes_b,
    }
}

fn nodes_in_zone(
    g: &HighwayGraph,
    route_id: &str,
    cx: f64,
    cy: f64,
    radius: f64,
) -> Vec<NodeIndex> {
    let edges = g.route_edges(route_id);
    let mut nodes = Vec::new();
    for &ei in edges {
        if let Some((s, t)) = g.graph.edge_endpoints(ei) {
            for ni in [s, t] {
                let c = &g.graph[ni].coord;
                if in_zone(c.x, c.y, cx, cy, radius) {
                    nodes.push(ni);
                }
            }
        }
    }
    nodes.sort();
    nodes.dedup();
    nodes
}

fn in_zone(lon: f64, lat: f64, cx: f64, cy: f64, radius: f64) -> bool {
    let dx = lon - cx;
    let dy = lat - cy;
    (dx * dx + dy * dy).sqrt() <= radius
}

/// Simplified k-connectivity: BFS-based count of independent path starts.
/// For small zones (50-mile radius), this approximates max-flow between zones.
fn compute_k_connectivity(
    g: &HighwayGraph,
    zone_a: &[NodeIndex],
    zone_b: &[NodeIndex],
    zone_edges: &HashSet<EdgeIndex>,
) -> usize {
    if zone_a.is_empty() || zone_b.is_empty() {
        return 0;
    }

    let zone_b_set: HashSet<NodeIndex> = zone_b.iter().cloned().collect();

    // Count paths from zone_a nodes to zone_b, using BFS with edge removal
    // (simplified: count unique first-hop edges that lead to zone_b)
    let mut k = 0;
    let mut used_edges: HashSet<EdgeIndex> = HashSet::new();

    for _ in 0..5 {
        // try to find up to 5 paths
        let path = bfs_path(g, zone_a, &zone_b_set, zone_edges, &used_edges);
        if path.is_empty() {
            break;
        }
        for ei in &path {
            used_edges.insert(*ei);
        }
        k += 1;
    }

    k
}

/// BFS from any zone_a node to any zone_b node, avoiding used edges.
fn bfs_path(
    g: &HighwayGraph,
    zone_a: &[NodeIndex],
    zone_b: &HashSet<NodeIndex>,
    zone_edges: &HashSet<EdgeIndex>,
    used_edges: &HashSet<EdgeIndex>,
) -> Vec<EdgeIndex> {
    let mut visited: HashMap<NodeIndex, Option<(NodeIndex, EdgeIndex)>> = HashMap::new();
    let mut queue = VecDeque::new();

    for &start in zone_a {
        visited.insert(start, None);
        queue.push_back(start);
    }

    let mut reached_b: Option<NodeIndex> = None;

    'outer: while let Some(u) = queue.pop_front() {
        if zone_b.contains(&u) {
            reached_b = Some(u);
            break;
        }

        // Both directions (bidirectional highway)
        for er in g.graph.edges(u) {
            let ei = er.id();
            if !zone_edges.contains(&ei) || used_edges.contains(&ei) {
                continue;
            }
            let v = er.target();
            if !visited.contains_key(&v) {
                visited.insert(v, Some((u, ei)));
                queue.push_back(v);
                if zone_b.contains(&v) {
                    reached_b = Some(v);
                    break 'outer;
                }
            }
        }
        for er in g.graph.edges_directed(u, petgraph::Direction::Incoming) {
            let ei = er.id();
            if !zone_edges.contains(&ei) || used_edges.contains(&ei) {
                continue;
            }
            let v = er.source();
            if !visited.contains_key(&v) {
                visited.insert(v, Some((u, ei)));
                queue.push_back(v);
                if zone_b.contains(&v) {
                    reached_b = Some(v);
                    break 'outer;
                }
            }
        }
    }

    // Reconstruct edge path
    let mut path = Vec::new();
    if let Some(mut cur) = reached_b {
        while let Some(Some((prev, ei))) = visited.get(&cur) {
            path.push(*ei);
            cur = *prev;
        }
    }
    path.reverse();
    path
}
