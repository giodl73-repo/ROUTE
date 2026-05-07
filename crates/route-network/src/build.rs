use crate::graph::{HighwayEdge, HighwayGraph, HighwayNode, JoinReport};
use geo_types::Coord;
use petgraph::graph::NodeIndex;
use route_data::{HpmsRecord, NhsSegment};
use rstar::RTree;
use std::collections::HashMap;

const NODE_SNAP_DEG: f64 = 0.0005; // ~50m at mid-latitudes — snap tolerance for node deduplication

/// Build a HighwayGraph from NHS segments with HPMS attributes joined by route+state key.
pub fn build_graph(
    segments: Vec<NhsSegment>,
    hpms: &[HpmsRecord],
) -> (HighwayGraph, JoinReport) {
    let mut g = HighwayGraph::new();
    let mut report = JoinReport::default();
    report.data_sparse_threshold = 3;

    // Build HPMS lookup: (route_id, state) → HpmsRecord
    let hpms_map: HashMap<(String, String), &HpmsRecord> = hpms
        .iter()
        .map(|r| ((normalise_route_id(&r.route_id), r.state.clone()), r))
        .collect();

    // Node deduplication: coord (snapped to NODE_SNAP_DEG grid) → NodeIndex
    let mut node_map: HashMap<(i64, i64), NodeIndex> = HashMap::new();

    let mut edge_id = 0u64;

    for seg in segments {
        let coords = seg.geometry.0.as_slice();
        if coords.len() < 2 {
            continue;
        }

        let start_ni = get_or_create_node(&mut g, &mut node_map, coords[0]);
        let end_ni = get_or_create_node(&mut g, &mut node_map, *coords.last().unwrap());

        // HPMS join
        let key = (normalise_route_id(&seg.route_id), seg.state.clone());
        let (aadt, pct_truck, iri, lane_count) = match hpms_map.get(&key) {
            Some(h) => (h.aadt, h.pct_truck, h.iri, h.lane_count),
            None => {
                report.hpms_failures += 1;
                (None, None, None, None)
            }
        };

        let edge = HighwayEdge {
            id: edge_id,
            route_id: normalise_route_id(&seg.route_id),
            state: seg.state.clone(),
            geometry: seg.geometry,
            length_miles: seg.length_miles,
            lane_count,
            aadt,
            pct_truck,
            iri,
            tti: None, // joined separately from HPMS FPM
            pti: None,
        };

        let ei = g.graph.add_edge(start_ni, end_ni, edge);
        g.route_index
            .entry(normalise_route_id(&seg.route_id))
            .or_default()
            .push(ei);

        edge_id += 1;
    }

    // Mark interchange nodes (connected to ≥2 routes)
    mark_interchanges(&mut g);

    // Flag data-sparse routes
    // (CorridorAttributes built later in route-score; check here is edge-level only)
    // Full data-sparse check runs after corridor attribute aggregation in route-score.

    (g, report)
}

fn get_or_create_node(
    g: &mut HighwayGraph,
    node_map: &mut HashMap<(i64, i64), NodeIndex>,
    coord: Coord<f64>,
) -> NodeIndex {
    let key = snap(coord);
    *node_map.entry(key).or_insert_with(|| {
        let id = g.graph.node_count() as u64;
        g.graph.add_node(HighwayNode {
            id,
            coord,
            is_interchange: false,
        })
    })
}

fn snap(coord: Coord<f64>) -> (i64, i64) {
    let scale = 1.0 / NODE_SNAP_DEG;
    ((coord.x * scale).round() as i64, (coord.y * scale).round() as i64)
}

fn mark_interchanges(g: &mut HighwayGraph) {
    // A node is an interchange if its incident edges include ≥2 distinct route_ids
    let node_ids: Vec<_> = g.graph.node_indices().collect();
    for ni in node_ids {
        let routes: std::collections::HashSet<&str> = g
            .graph
            .edges(ni)
            .map(|er| er.weight().route_id.as_str())
            .collect();
        if routes.len() >= 2 {
            g.graph[ni].is_interchange = true;
        }
    }
}

/// Normalise route IDs: strip spaces and dashes, uppercase.
/// "I-80" → "I80", "i 95" → "I95"
pub fn normalise_route_id(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}
