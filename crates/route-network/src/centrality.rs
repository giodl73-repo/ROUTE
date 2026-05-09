use crate::graph::HighwayGraph;
use petgraph::graph::EdgeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

/// Compute weighted Brandes betweenness centrality for all edges.
///
/// Uses Brandes (2001) algorithm with edge-level accumulation and shortest-path
/// multiplicity (`sigma`) tracking.
/// For 5,000 nodes and 6,000 edges this completes in a few seconds.
///
/// NOTE: Only meaningful when the full national graph is loaded. Callers should
/// leave B2 unset for single-corridor scoring and populate it only from
/// `score-all`/calibration runs over the national graph.
pub fn compute_edge_betweenness(g: &HighwayGraph) -> HashMap<EdgeIndex, f64> {
    let node_count = g.graph.node_count();
    if node_count == 0 {
        return HashMap::new();
    }

    let mut raw: HashMap<EdgeIndex, f64> = HashMap::new();

    // Brandes algorithm: for each source node, single-source shortest paths,
    // then back-propagate dependency.
    let nodes: Vec<_> = g.graph.node_indices().collect();

    for &source in &nodes {
        // Single-source shortest paths via Dijkstra (weighted by miles)
        let (dist, pred, sigma) = dijkstra_with_pred(g, source);

        // Back-propagation: accumulate edge dependency
        let mut dep: HashMap<petgraph::graph::NodeIndex, f64> = HashMap::new();

        // Process nodes in reverse order of distance
        let mut ordered: Vec<_> = dist.iter().collect();
        ordered.sort_by(|a, b| b.1.total_cmp(a.1)); // reverse distance order

        for (&w, _) in &ordered {
            if let Some(predecessors) = pred.get(&w) {
                let sigma_w = sigma.get(&w).copied().unwrap_or(0.0);
                if sigma_w <= 0.0 {
                    continue;
                }
                let delta_w = dep.get(&w).cloned().unwrap_or(0.0);
                for &(v, ei) in predecessors {
                    let sigma_v = sigma.get(&v).copied().unwrap_or(0.0);
                    let contribution = (sigma_v / sigma_w) * (1.0 + delta_w);
                    *dep.entry(v).or_insert(0.0) += contribution;
                    *raw.entry(ei).or_insert(0.0) += contribution;
                }
            }
        }
    }

    // Normalise to 0.0–1.0
    let max = raw.values().cloned().fold(0.0f64, f64::max);
    if max > 0.0 {
        raw.values_mut().for_each(|v| *v /= max);
    }

    raw
}

/// Single-source Dijkstra returning distances, predecessor edges, and shortest-path counts.
fn dijkstra_with_pred(
    g: &HighwayGraph,
    source: petgraph::graph::NodeIndex,
) -> (
    HashMap<petgraph::graph::NodeIndex, f64>,
    HashMap<petgraph::graph::NodeIndex, Vec<(petgraph::graph::NodeIndex, EdgeIndex)>>,
    HashMap<petgraph::graph::NodeIndex, f64>,
) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist: HashMap<petgraph::graph::NodeIndex, f64> = HashMap::new();
    let mut pred: HashMap<
        petgraph::graph::NodeIndex,
        Vec<(petgraph::graph::NodeIndex, EdgeIndex)>,
    > = HashMap::new();
    let mut sigma: HashMap<petgraph::graph::NodeIndex, f64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(ordered_float::NotNan<f64>, petgraph::graph::NodeIndex)>> =
        BinaryHeap::new();

    dist.insert(source, 0.0);
    sigma.insert(source, 1.0);
    heap.push(Reverse((ordered_float::NotNan::new(0.0).unwrap(), source)));

    while let Some(Reverse((cost, u))) = heap.pop() {
        let cost = cost.into_inner();
        if cost > *dist.get(&u).unwrap_or(&f64::MAX) + 1e-9 {
            continue;
        }
        for er in g.graph.edges(u) {
            let v = er.target();
            let edge_miles = er.weight().length_miles;
            if !edge_miles.is_finite() || edge_miles < 0.0 {
                continue;
            }
            let new_cost = cost + edge_miles;
            let prev = dist.get(&v).cloned().unwrap_or(f64::MAX);
            if new_cost < prev - 1e-9 {
                dist.insert(v, new_cost);
                pred.insert(v, vec![(u, er.id())]);
                sigma.insert(v, sigma.get(&u).copied().unwrap_or(0.0));
                if let Ok(cost) = ordered_float::NotNan::new(new_cost) {
                    heap.push(Reverse((cost, v)));
                }
            } else if (new_cost - prev).abs() < 1e-9 {
                pred.entry(v).or_default().push((u, er.id()));
                *sigma.entry(v).or_insert(0.0) += sigma.get(&u).copied().unwrap_or(0.0);
            }
        }
    }

    (dist, pred, sigma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HighwayEdge, HighwayGraph, HighwayNode};
    use geo_types::{coord, LineString};
    use petgraph::graph::{EdgeIndex, NodeIndex};

    fn add_node(g: &mut HighwayGraph, id: u64, x: f64) -> NodeIndex {
        g.graph.add_node(HighwayNode {
            id,
            coord: coord! { x: x, y: 0.0 },
            is_interchange: false,
        })
    }

    fn add_edge(
        g: &mut HighwayGraph,
        id: u64,
        from: NodeIndex,
        to: NodeIndex,
        length_miles: f64,
    ) -> EdgeIndex {
        g.graph.add_edge(
            from,
            to,
            HighwayEdge {
                id,
                route_id: format!("I{id}"),
                state: "TS".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: g.graph[from].coord.x, y: 0.0 },
                    coord! { x: g.graph[to].coord.x, y: 0.0 },
                ]),
                length_miles,
                lane_count: None,
                aadt: None,
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        )
    }

    #[test]
    fn equal_shortest_paths_split_edge_dependency() {
        let mut g = HighwayGraph::new();
        let s = add_node(&mut g, 1, 0.0);
        let a = add_node(&mut g, 2, 1.0);
        let b = add_node(&mut g, 3, 1.0);
        let t = add_node(&mut g, 4, 2.0);

        let sa = add_edge(&mut g, 1, s, a, 1.0);
        let at = add_edge(&mut g, 2, a, t, 1.0);
        let sb = add_edge(&mut g, 3, s, b, 1.0);
        let bt = add_edge(&mut g, 4, b, t, 1.0);

        let centrality = compute_edge_betweenness(&g);

        let upper = centrality[&sa] + centrality[&at];
        let lower = centrality[&sb] + centrality[&bt];
        assert!((upper - lower).abs() < 1e-9);
        assert!(centrality[&sa] > 0.0);
    }

    #[test]
    fn non_shortest_direct_edge_receives_no_dependency() {
        let mut g = HighwayGraph::new();
        let s = add_node(&mut g, 1, 0.0);
        let a = add_node(&mut g, 2, 1.0);
        let t = add_node(&mut g, 3, 2.0);

        let sa = add_edge(&mut g, 1, s, a, 1.0);
        let at = add_edge(&mut g, 2, a, t, 1.0);
        let st = add_edge(&mut g, 3, s, t, 10.0);

        let centrality = compute_edge_betweenness(&g);

        assert!(centrality[&sa] > centrality.get(&st).copied().unwrap_or(0.0));
        assert!(centrality[&at] > centrality.get(&st).copied().unwrap_or(0.0));
    }
}
