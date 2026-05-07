/// Spatial join helpers — snap NBI bridges and population data onto corridor edges.
use crate::graph::HighwayGraph;
use petgraph::graph::EdgeIndex;
use rstar::{RTree, RTreeObject, AABB};
use route_data::NbiRecord;
use std::collections::HashMap;

/// A point in the R-tree with an associated payload ID.
struct IndexedPoint {
    coord: [f64; 2],
    id: usize,
}

impl RTreeObject for IndexedPoint {
    type Envelope = AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.coord)
    }
}

impl rstar::PointDistance for IndexedPoint {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let dx = self.coord[0] - point[0];
        let dy = self.coord[1] - point[1];
        dx * dx + dy * dy
    }
}

/// Join NBI bridge records to corridor edges.
/// Tolerance: ≤0.002° (~170m). Route-name similarity check: NBI.FACILITY_CARRIED_007
/// must contain the interstate number (e.g. "80" for I-80).
///
/// Returns: map of EdgeIndex → Vec of matching NBI records.
pub fn join_nbi_to_edges(
    g: &HighwayGraph,
    nbi: &[NbiRecord],
) -> (HashMap<EdgeIndex, Vec<usize>>, usize) {
    const TOLERANCE_DEG: f64 = 0.002;
    const TOLERANCE_DEG_SQ: f64 = TOLERANCE_DEG * TOLERANCE_DEG;

    // Build R-tree of NBI points
    let nbi_points: Vec<IndexedPoint> = nbi
        .iter()
        .enumerate()
        .map(|(i, r)| IndexedPoint { coord: [r.lon, r.lat], id: i })
        .collect();
    let tree = RTree::bulk_load(nbi_points);

    let mut result: HashMap<EdgeIndex, Vec<usize>> = HashMap::new();
    let mut failures = 0usize;

    for ei in g.graph.edge_indices() {
        let edge = &g.graph[ei];
        let interstate_num = extract_interstate_num(&edge.route_id);

        // Sample midpoint of edge geometry for the proximity search
        let geom = &edge.geometry;
        let mid_idx = geom.0.len() / 2;
        let mid = geom.0[mid_idx];
        let query_pt = [mid.x, mid.y];

        let nearby: Vec<usize> = tree
            .locate_within_distance(query_pt, TOLERANCE_DEG_SQ)
            .filter(|p| {
                // Route-name similarity: NBI facility carried must mention the interstate number
                interstate_num.as_deref().map_or(true, |num| {
                    nbi[p.id].route_on_bridge.contains(num)
                })
            })
            .map(|p| p.id)
            .collect();

        if nearby.is_empty() {
            failures += 1;
        } else {
            result.insert(ei, nearby);
        }
    }

    (result, failures)
}

/// Extract the numeric part of an interstate route ID.
/// "I80" → Some("80"), "US30" → None
fn extract_interstate_num(route_id: &str) -> Option<String> {
    if route_id.starts_with('I') {
        Some(route_id[1..].to_string())
    } else {
        None
    }
}
