use crate::graph::HighwayGraph;
use petgraph::graph::EdgeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

/// Compute approximate betweenness centrality for all edges.
///
/// Uses Brandes (2001) algorithm with edge-level accumulation.
/// For 5,000 nodes and 6,000 edges this completes in a few seconds.
///
/// NOTE: Only meaningful when the full national graph is loaded.
/// Mark B2 as `estimated: true` until `score-all` runs.
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
        let (dist, pred) = dijkstra_with_pred(g, source);

        // Back-propagation: accumulate edge dependency
        let mut dep: HashMap<petgraph::graph::NodeIndex, f64> = HashMap::new();

        // Process nodes in reverse order of distance
        let mut ordered: Vec<_> = dist.iter().collect();
        ordered.sort_by(|a, b| b.1.total_cmp(a.1)); // reverse distance order

        for (&w, _) in &ordered {
            if let Some(predecessors) = pred.get(&w) {
                let sigma_w = 1.0; // simplified: assume one shortest path per node pair
                let delta_w = dep.get(&w).cloned().unwrap_or(0.0);
                for &(v, ei) in predecessors {
                    let contribution = (sigma_w / sigma_w) * (1.0 + delta_w);
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

/// Single-source Dijkstra returning distances and predecessor edges.
/// Returns (dist: NodeIndex → f64, pred: NodeIndex → Vec<(NodeIndex, EdgeIndex)>)
fn dijkstra_with_pred(
    g: &HighwayGraph,
    source: petgraph::graph::NodeIndex,
) -> (
    HashMap<petgraph::graph::NodeIndex, f64>,
    HashMap<petgraph::graph::NodeIndex, Vec<(petgraph::graph::NodeIndex, EdgeIndex)>>,
) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist: HashMap<petgraph::graph::NodeIndex, f64> = HashMap::new();
    let mut pred: HashMap<
        petgraph::graph::NodeIndex,
        Vec<(petgraph::graph::NodeIndex, EdgeIndex)>,
    > = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(ordered_float::NotNan<f64>, petgraph::graph::NodeIndex)>> =
        BinaryHeap::new();

    dist.insert(source, 0.0);
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
                if let Ok(cost) = ordered_float::NotNan::new(new_cost) {
                    heap.push(Reverse((cost, v)));
                }
            } else if (new_cost - prev).abs() < 1e-9 {
                pred.entry(v).or_default().push((u, er.id()));
            }
        }
    }

    (dist, pred)
}
