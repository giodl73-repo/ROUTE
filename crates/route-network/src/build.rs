use crate::graph::{HighwayEdge, HighwayGraph, HighwayNode, JoinReport};
use geo_types::Coord;
use petgraph::graph::NodeIndex;
use route_data::{HpmsRecord, NhsSegment};
use std::collections::HashMap;

const NODE_SNAP_DEG: f64 = 0.0005; // ~50m at mid-latitudes — snap tolerance for node deduplication

/// Build a HighwayGraph from NHS segments with HPMS attributes joined by route+state key.
pub fn build_graph(segments: Vec<NhsSegment>, hpms: &[HpmsRecord]) -> (HighwayGraph, JoinReport) {
    let mut g = HighwayGraph::new();
    let mut report = JoinReport::default();
    report.data_sparse_threshold = 3;

    // Build HPMS lookup: route_id → aggregated values across all states
    // TIGER national file has no state field, so join by route_id only.
    // Aggregate: median AADT (not mean — avoids outlier states skewing), median IRI,
    // modal lane count, mean pct_truck, modal speed_limit.
    let hpms_map: HashMap<String, HpmsAgg> = aggregate_hpms_by_route(hpms);

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

        // HPMS join by route_id (national aggregation — no state field in TIGER)
        let norm_id = normalise_route_id(&seg.route_id);
        let (aadt, pct_truck, iri, lane_count, speed_limit) = match hpms_map.get(&norm_id) {
            Some(h) => (h.aadt, h.pct_truck, h.iri, h.lane_count, h.speed_limit),
            None => {
                report.hpms_failures += 1;
                (None, None, None, None, None)
            }
        };

        let edge = HighwayEdge {
            id: edge_id,
            route_id: norm_id.clone(),
            state: seg.state.clone(),
            road_class: seg.road_class,
            geometry: seg.geometry,
            length_miles: seg.length_miles,
            lane_count,
            aadt,
            pct_truck,
            iri,
            tti: None,
            pti: None,
            speed_limit,
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
    (
        (coord.x * scale).round() as i64,
        (coord.y * scale).round() as i64,
    )
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

/// Aggregated HPMS values for a single route across all states.
#[derive(Debug)]
pub struct HpmsAgg {
    pub aadt: Option<u32>,
    pub pct_truck: Option<f32>,
    pub lane_count: Option<u8>,
    pub iri: Option<f32>,
    pub speed_limit: Option<u8>,
}

/// Aggregate HPMS records by route_id across all states.
/// Uses median AADT (robust to outlier states), mean pct_truck, modal lane_count.
pub fn aggregate_hpms_by_route(hpms: &[route_data::HpmsRecord]) -> HashMap<String, HpmsAgg> {
    // Group records by normalised route_id
    let mut groups: HashMap<String, Vec<&route_data::HpmsRecord>> = HashMap::new();
    for r in hpms {
        let id = normalise_route_id(&r.route_id);
        groups.entry(id).or_default().push(r);
    }

    groups
        .into_iter()
        .map(|(id, records)| {
            let aadts: Vec<u32> = records.iter().filter_map(|r| r.aadt).collect();
            let pcts: Vec<f32> = records.iter().filter_map(|r| r.pct_truck).collect();
            let lanes: Vec<u8> = records.iter().filter_map(|r| r.lane_count).collect();
            let iris: Vec<f32> = records.iter().filter_map(|r| r.iri).collect();
            let speeds: Vec<u8> = records.iter().filter_map(|r| r.speed_limit).collect();

            let agg = HpmsAgg {
                aadt: median_u32(&aadts),
                pct_truck: mean_f32(&pcts),
                lane_count: mode_u8(&lanes),
                iri: median_f32(&iris),
                speed_limit: mode_u8(&speeds),
            };
            (id, agg)
        })
        .collect()
}

fn median_u32(v: &[u32]) -> Option<u32> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort();
    Some(s[s.len() / 2])
}

fn median_f32(v: &[f32]) -> Option<f32> {
    let mut s: Vec<f32> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if s.is_empty() {
        return None;
    }
    s.sort_by(f32::total_cmp);
    Some(s[s.len() / 2])
}

fn mean_f32(v: &[f32]) -> Option<f32> {
    let vals: Vec<f32> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum::<f32>() / vals.len() as f32)
}

fn mode_u8(v: &[u8]) -> Option<u8> {
    if v.is_empty() {
        return None;
    }
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for &x in v {
        *counts.entry(x).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|&(_, c)| c).map(|(k, _)| k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{Coord, LineString};
    use route_data::{HpmsRecord, NhsSegment, RoadClass};

    fn seg(route_id: &str, x0: f64, x1: f64) -> NhsSegment {
        NhsSegment {
            route_id: route_id.to_string(),
            state: "NE".to_string(),
            road_class: RoadClass::Interstate,
            length_miles: 10.0,
            geometry: LineString::from(vec![Coord { x: x0, y: 41.0 }, Coord { x: x1, y: 41.0 }]),
            nhs_type: 2,
        }
    }

    fn hpms(route_id: &str, aadt: u32, pct_truck: f32, lanes: u8) -> HpmsRecord {
        HpmsRecord {
            state: "NE".to_string(),
            route_id: route_id.to_string(),
            aadt: Some(aadt),
            pct_truck: Some(pct_truck),
            lane_count: Some(lanes),
            iri: Some(1.5),
            speed_limit: Some(70),
        }
    }

    #[test]
    fn normalise_route_id_strips_separators_and_uppercases() {
        assert_eq!(normalise_route_id("I-80"), "I80");
        assert_eq!(normalise_route_id("i 95"), "I95");
        assert_eq!(normalise_route_id("US-30"), "US30");
    }

    #[test]
    fn build_graph_joins_hpms_by_normalised_route() {
        let (g, report) = build_graph(
            vec![seg("I-80", -101.0, -100.0), seg("I 80", -100.0, -99.0)],
            &[hpms("I80", 42_000, 0.18, 4)],
        );

        assert_eq!(report.hpms_failures, 0);
        assert_eq!(g.route_edges("I80").len(), 2);
        for &ei in g.route_edges("I80") {
            assert_eq!(g.graph[ei].aadt, Some(42_000));
            assert_eq!(g.graph[ei].lane_count, Some(4));
        }
    }

    #[test]
    fn aggregate_hpms_ignores_non_finite_float_inputs() {
        let records = vec![
            HpmsRecord {
                state: "A".into(),
                route_id: "I80".into(),
                aadt: Some(10_000),
                pct_truck: Some(f32::NAN),
                lane_count: Some(2),
                iri: Some(f32::NAN),
                speed_limit: Some(65),
            },
            hpms("I80", 30_000, 0.20, 4),
        ];

        let agg = aggregate_hpms_by_route(&records);
        let i80 = agg.get("I80").unwrap();
        assert_eq!(i80.aadt, Some(30_000));
        assert_eq!(i80.pct_truck, Some(0.20));
        assert_eq!(i80.iri, Some(1.5));
    }
}
