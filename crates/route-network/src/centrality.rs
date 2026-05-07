use crate::graph::HighwayGraph;
use petgraph::graph::EdgeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

/// Compute approximate betweenness centrality for all edges using the Brandes algorithm.
///
/// NOTE: Only meaningful when the full national graph is loaded (`route score-all`).
/// Partial-graph centrality is misleading — corridors that appear central in a 20-route
/// graph may not be central nationally. Mark B2 as estimated until score-all completes.
///
/// Returns a map of EdgeIndex → normalised centrality (0.0–1.0 within this graph).
pub fn compute_edge_betweenness(g: &HighwayGraph) -> HashMap<EdgeIndex, f64> {
    // Use petgraph's built-in Dijkstra for shortest paths; approximate Brandes over
    // a sample of source nodes for large graphs.
    let node_count = g.graph.node_count();
    if node_count == 0 {
        return HashMap::new();
    }

    let mut raw: HashMap<EdgeIndex, f64> = HashMap::new();

    // For each source node, run single-source shortest paths (Dijkstra weighted by miles)
    // and accumulate edge dependency.
    // For the full national graph (~50k nodes), this is O(N·E·log N) — parallelise with Rayon.
    use rayon::prelude::*;

    let nodes: Vec<_> = g.graph.node_indices().collect();
    let contributions: Vec<HashMap<EdgeIndex, f64>> = nodes
        .par_iter()
        .map(|&source| single_source_dependency(g, source))
        .collect();

    for contrib in contributions {
        for (ei, val) in contrib {
            *raw.entry(ei).or_insert(0.0) += val;
        }
    }

    // Normalise to 0.0–1.0 range
    let max = raw.values().cloned().fold(f64::NEG_INFINITY, f64::max);
    if max > 0.0 {
        raw.values_mut().for_each(|v| *v /= max);
    }

    raw
}

fn single_source_dependency(
    g: &HighwayGraph,
    source: petgraph::graph::NodeIndex,
) -> HashMap<EdgeIndex, f64> {
    use petgraph::algo::dijkstra;

    let dist = dijkstra(
        &g.graph,
        source,
        None,
        |er| er.weight().length_miles,
    );

    // Simplified dependency accumulation — full Brandes requires predecessor tracking.
    // TODO: implement full Brandes predecessor-based dependency for accuracy.
    // This stub returns uniform small contributions as a placeholder.
    let mut dep = HashMap::new();
    for ei in g.graph.edge_indices() {
        let er = g.graph.edge_endpoints(ei).unwrap();
        if dist.contains_key(&er.0) && dist.contains_key(&er.1) {
            *dep.entry(ei).or_insert(0.0) += 1.0;
        }
    }
    dep
}
