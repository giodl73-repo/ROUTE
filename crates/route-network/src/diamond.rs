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
use crate::tier::T1_BACKBONE_ROUTES;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet, VecDeque};

const DIAMOND_RADIUS_DEG: f64 = 0.7; // ~50 miles at mid-latitude

struct CuratedT1Intersection {
    name: &'static str,
    route_a: &'static str,
    route_b: &'static str,
    lon: f64,
    lat: f64,
}

// TIGER primary-road segments do not always share an endpoint at major interchanges.
// Keep scenario-backed T1/T1 anchors here so pressure tests can bind to the same
// locations as the standards and failure ledgers.
const CURATED_T1_INTERSECTIONS: &[CuratedT1Intersection] = &[
    CuratedT1Intersection {
        name: "I-80/I-90",
        route_a: "I80",
        route_b: "I90",
        lon: -84.8,
        lat: 41.6,
    },
    CuratedT1Intersection {
        name: "I-35/I-80",
        route_a: "I35",
        route_b: "I80",
        lon: -93.573,
        lat: 41.659,
    },
    CuratedT1Intersection {
        name: "I-35/I-40",
        route_a: "I35",
        route_b: "I40",
        lon: -97.53,
        lat: 35.46,
    },
    CuratedT1Intersection {
        name: "I-40/I-75",
        route_a: "I40",
        route_b: "I75",
        lon: -84.05,
        lat: 35.9,
    },
    CuratedT1Intersection {
        name: "I-10/I-35",
        route_a: "I10",
        route_b: "I35",
        lon: -98.5,
        lat: 29.43,
    },
    CuratedT1Intersection {
        name: "I-75/I-80",
        route_a: "I75",
        route_b: "I80",
        lon: -83.65,
        lat: 41.55,
    },
    CuratedT1Intersection {
        name: "I-90/I-95",
        route_a: "I90",
        route_b: "I95",
        lon: -71.26,
        lat: 42.35,
    },
    CuratedT1Intersection {
        name: "I-10/I-95",
        route_a: "I10",
        route_b: "I95",
        lon: -81.66,
        lat: 30.32,
    },
    CuratedT1Intersection {
        name: "I-5/I-10",
        route_a: "I5",
        route_b: "I10",
        lon: -118.23,
        lat: 34.05,
    },
    CuratedT1Intersection {
        name: "I-5/I-80",
        route_a: "I5",
        route_b: "I80",
        lon: -121.5,
        lat: 38.58,
    },
    CuratedT1Intersection {
        name: "I-5/I-90",
        route_a: "I5",
        route_b: "I90",
        lon: -122.33,
        lat: 47.59,
    },
    CuratedT1Intersection {
        name: "I-35/I-90",
        route_a: "I35",
        route_b: "I90",
        lon: -93.37,
        lat: 43.65,
    },
    CuratedT1Intersection {
        name: "I-40/I-95",
        route_a: "I40",
        route_b: "I95",
        lon: -78.55,
        lat: 35.38,
    },
    CuratedT1Intersection {
        name: "I-75/I-90",
        route_a: "I75",
        route_b: "I90",
        lon: -83.65,
        lat: 41.55,
    },
    CuratedT1Intersection {
        name: "I-5/I-40",
        route_a: "I5",
        route_b: "I40",
        lon: -117.02,
        lat: 34.9,
    },
];

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
                .filter(|id| T1_BACKBONE_ROUTES.contains(&id.as_str()))
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

    for curated in CURATED_T1_INTERSECTIONS {
        let key = pair_key(curated.route_a, curated.route_b);
        if seen.contains(&key) {
            continue;
        }
        if g.route_edges(curated.route_a).is_empty() || g.route_edges(curated.route_b).is_empty() {
            continue;
        }
        let Some(center) = nearest_node(g, curated.lon, curated.lat) else {
            continue;
        };
        seen.insert(key);
        intersections.push(T1Intersection {
            name: curated.name.to_string(),
            route_a: curated.route_a.to_string(),
            route_b: curated.route_b.to_string(),
            center,
            lon: curated.lon,
            lat: curated.lat,
        });
    }

    intersections
}

fn pair_key(route_a: &str, route_b: &str) -> String {
    let mut routes = [route_a, route_b];
    routes.sort();
    format!("{}x{}", routes[0], routes[1])
}

fn nearest_node(g: &HighwayGraph, lon: f64, lat: f64) -> Option<NodeIndex> {
    g.graph.node_indices().min_by(|&a, &b| {
        let ca = &g.graph[a].coord;
        let cb = &g.graph[b].coord;
        let da = squared_distance(ca.x, ca.y, lon, lat);
        let db = squared_distance(cb.x, cb.y, lon, lat);
        da.total_cmp(&db)
    })
}

fn squared_distance(lon: f64, lat: f64, target_lon: f64, target_lat: f64) -> f64 {
    let dx = lon - target_lon;
    let dy = lat - target_lat;
    dx * dx + dy * dy
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

#[cfg(test)]
mod tests {
    use super::{
        compute_k_connectivity, find_intersection, find_t1_intersections, CURATED_T1_INTERSECTIONS,
    };
    use crate::graph::{HighwayEdge, HighwayGraph, HighwayNode};
    use crate::tier::T1_BACKBONE_ROUTES;
    use geo_types::{coord, LineString};
    use petgraph::graph::{EdgeIndex, NodeIndex};
    use std::collections::HashSet;

    fn node(id: u64, x: f64, y: f64) -> HighwayNode {
        HighwayNode {
            id,
            coord: coord! { x: x, y: y },
            is_interchange: false,
        }
    }

    fn edge(id: u64, route_id: &str) -> HighwayEdge {
        HighwayEdge {
            id,
            route_id: route_id.to_string(),
            state: "TS".to_string(),
            road_class: route_data::RoadClass::Interstate,
            geometry: LineString::from(vec![coord! { x: 0.0, y: 0.0 }, coord! { x: 1.0, y: 0.0 }]),
            length_miles: 1.0,
            lane_count: Some(2),
            aadt: None,
            pct_truck: None,
            iri: None,
            tti: None,
            pti: None,
            speed_limit: Some(65),
        }
    }

    fn add_path(
        graph: &mut HighwayGraph,
        source: NodeIndex,
        sink: NodeIndex,
        route_id: &str,
        edge_id: &mut u64,
    ) -> Vec<EdgeIndex> {
        let mid = graph
            .graph
            .add_node(node(*edge_id + 1_000, *edge_id as f64, 1.0));
        let first = graph.graph.add_edge(source, mid, edge(*edge_id, route_id));
        *edge_id += 1;
        let second = graph.graph.add_edge(mid, sink, edge(*edge_id, route_id));
        *edge_id += 1;
        vec![first, second]
    }

    #[test]
    fn k_connectivity_returns_zero_when_either_zone_is_empty() {
        let mut graph = HighwayGraph::new();
        let source = graph.graph.add_node(node(1, 0.0, 0.0));
        let zone_edges = HashSet::new();

        assert_eq!(
            compute_k_connectivity(&graph, &[source], &[], &zone_edges),
            0
        );
        assert_eq!(
            compute_k_connectivity(&graph, &[], &[source], &zone_edges),
            0
        );
    }

    #[test]
    fn k_connectivity_counts_single_edge_disjoint_path() {
        let mut graph = HighwayGraph::new();
        let source = graph.graph.add_node(node(1, 0.0, 0.0));
        let sink = graph.graph.add_node(node(2, 2.0, 0.0));
        let mut edge_id = 10;
        let zone_edges: HashSet<_> = add_path(&mut graph, source, sink, "I35", &mut edge_id)
            .into_iter()
            .collect();

        let k = compute_k_connectivity(&graph, &[source], &[sink], &zone_edges);

        assert_eq!(k, 1);
    }

    #[test]
    fn k_connectivity_counts_parallel_edge_disjoint_paths() {
        let mut graph = HighwayGraph::new();
        let source = graph.graph.add_node(node(1, 0.0, 0.0));
        let sink = graph.graph.add_node(node(2, 2.0, 0.0));
        let mut edge_id = 10;
        let mut zone_edges = HashSet::new();
        zone_edges.extend(add_path(&mut graph, source, sink, "I35", &mut edge_id));
        zone_edges.extend(add_path(&mut graph, source, sink, "I80", &mut edge_id));
        zone_edges.extend(add_path(&mut graph, source, sink, "I90", &mut edge_id));

        let k = compute_k_connectivity(&graph, &[source], &[sink], &zone_edges);

        assert_eq!(k, 3);
    }

    #[test]
    fn k_connectivity_respects_zone_edge_filter() {
        let mut graph = HighwayGraph::new();
        let source = graph.graph.add_node(node(1, 0.0, 0.0));
        let sink = graph.graph.add_node(node(2, 2.0, 0.0));
        let mut edge_id = 10;
        let included = add_path(&mut graph, source, sink, "I35", &mut edge_id);
        let excluded = add_path(&mut graph, source, sink, "I80", &mut edge_id);
        let mut zone_edges: HashSet<_> = included.into_iter().collect();
        zone_edges.insert(excluded[0]);

        let k = compute_k_connectivity(&graph, &[source], &[sink], &zone_edges);

        assert_eq!(k, 1);
    }

    #[test]
    fn curated_des_moines_anchor_is_available_when_routes_do_not_share_endpoint() {
        let mut graph = HighwayGraph::new();
        let i35_south = graph.graph.add_node(node(1, -93.58, 41.1));
        let i35_north = graph.graph.add_node(node(2, -93.58, 42.1));
        let i80_west = graph.graph.add_node(node(3, -94.2, 41.66));
        let i80_east = graph.graph.add_node(node(4, -93.0, 41.66));
        let i35 = graph.graph.add_edge(i35_south, i35_north, edge(1, "I35"));
        let i80 = graph.graph.add_edge(i80_west, i80_east, edge(2, "I80"));
        graph.route_index.insert("I35".to_string(), vec![i35]);
        graph.route_index.insert("I80".to_string(), vec![i80]);

        let intersection = find_intersection(&graph, "I35xI80").expect("curated Des Moines anchor");

        assert_eq!(intersection.name, "I-35/I-80");
        assert_eq!(intersection.route_a, "I35");
        assert_eq!(intersection.route_b, "I80");
    }

    #[test]
    fn curated_anchor_is_listed_once() {
        let mut graph = HighwayGraph::new();
        let shared = graph.graph.add_node(node(1, -93.573, 41.659));
        let i35_north = graph.graph.add_node(node(2, -93.58, 42.1));
        let i80_east = graph.graph.add_node(node(3, -93.0, 41.66));
        let i35 = graph.graph.add_edge(shared, i35_north, edge(1, "I35"));
        let i80 = graph.graph.add_edge(shared, i80_east, edge(2, "I80"));
        graph.route_index.insert("I35".to_string(), vec![i35]);
        graph.route_index.insert("I80".to_string(), vec![i80]);

        let matches = find_t1_intersections(&graph)
            .into_iter()
            .filter(|ix| {
                (ix.route_a == "I35" && ix.route_b == "I80")
                    || (ix.route_a == "I80" && ix.route_b == "I35")
            })
            .count();

        assert_eq!(matches, 1);
    }

    #[test]
    fn curated_anchor_catalog_covers_all_known_t1_pairs() {
        let mut graph = HighwayGraph::new();
        let mut edge_id = 1;
        for route_id in T1_BACKBONE_ROUTES {
            let source = graph
                .graph
                .add_node(node(edge_id, edge_id as f64, edge_id as f64));
            edge_id += 1;
            let sink = graph
                .graph
                .add_node(node(edge_id, edge_id as f64, edge_id as f64));
            let route_edge = graph.graph.add_edge(source, sink, edge(edge_id, route_id));
            graph
                .route_index
                .insert((*route_id).to_string(), vec![route_edge]);
            edge_id += 1;
        }

        let intersections = find_t1_intersections(&graph);
        for curated in CURATED_T1_INTERSECTIONS {
            assert!(
                intersections.iter().any(|ix| {
                    (ix.route_a == curated.route_a && ix.route_b == curated.route_b)
                        || (ix.route_a == curated.route_b && ix.route_b == curated.route_a)
                }),
                "missing curated pair {}x{}",
                curated.route_a,
                curated.route_b
            );
        }
    }
}
