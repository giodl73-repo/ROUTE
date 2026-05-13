/// Build CorridorAttributes from graph edges for a single corridor.
/// Only uses data already present in the graph — no external joins here.
/// Fields requiring HPMS/NBI/Census etc. stay None until joined separately.
use crate::corridor::{Corridor, CorridorAttributes};
use crate::graph::{HighwayEdge, HighwayGraph};
use petgraph::graph::EdgeIndex;

pub fn aggregate_corridor(g: &HighwayGraph, route_id: &str) -> Option<Corridor> {
    let edges = g.route_edges(route_id);
    if edges.is_empty() {
        return None;
    }

    let total_miles = corridor_total_miles(g, edges);
    let edge_count = edges.len();
    let is_upgrade = edges
        .first()
        .map(|&ei| g.graph[ei].road_class != route_data::RoadClass::Interstate)
        .unwrap_or(false);

    // States — collect from edges (may be empty for TIGER source)
    let mut states: Vec<String> = edges
        .iter()
        .map(|&ei| g.graph[ei].state.clone())
        .filter(|s| !s.is_empty())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    if states.is_empty() {
        states = infer_states_from_geometry(g, edges);
    }
    states.sort();

    // Termini — westernmost and easternmost nodes from geometry
    let termini = find_termini(g, edges);

    // AADT and lane aggregation
    let aadts: Vec<f64> = edges
        .iter()
        .filter_map(|&ei| g.graph[ei].aadt.map(|a| a as f64))
        .collect();
    let (p90_aadt, mean_aadt) = percentile_and_mean(&aadts);

    let lane_counts: Vec<f32> = edges
        .iter()
        .filter_map(|&ei| g.graph[ei].lane_count.map(|l| l as f32))
        .collect();
    let mean_lane_count = mean_f32(&lane_counts);

    let speed_limits: Vec<f32> = edges
        .iter()
        .filter_map(|&ei| g.graph[ei].speed_limit.map(|s| s as f32))
        .collect();
    let mean_speed_limit = mean_f32(&speed_limits);

    // Throughput capacity: lane_count × 1,900 pcph × 24h (rough daily capacity)
    let daily_capacity = mean_lane_count.map(|l| l as f64 * 1_900.0 * 24.0);
    let vc_ratio_p90 = p90_aadt.zip(daily_capacity).map(|(v, c)| (v / c) as f32);

    let pct_trucks: Vec<f32> = edges
        .iter()
        .filter_map(|&ei| g.graph[ei].pct_truck)
        .collect();
    let mean_pct_truck = mean_f32(&pct_trucks);

    let iris: Vec<f32> = edges.iter().filter_map(|&ei| g.graph[ei].iri).collect();
    let mean_iri = mean_f32(&iris);

    let ptis: Vec<f32> = edges.iter().filter_map(|&ei| g.graph[ei].pti).collect();
    let p90_pti = percentile_f32(&ptis, 0.90);

    let ttis: Vec<f32> = edges.iter().filter_map(|&ei| g.graph[ei].tti).collect();
    let mean_tti = mean_f32(&ttis);

    // Betweenness centrality — P90 of corridor edges (not mean)
    // Mean dilutes long rural corridors: I-80's rural WY/NE edges have near-zero
    // betweenness but its Chicago/Bay Area segments are highly central.
    // P90 captures the "spine" sections without extreme-outlier sensitivity.
    let betweenness_centrality = g.edge_betweenness.as_ref().and_then(|bc| {
        let mut vals: Vec<f64> = edges.iter().filter_map(|ei| bc.get(ei)).cloned().collect();
        if vals.is_empty() {
            return None;
        }
        vals.retain(|v| v.is_finite());
        if vals.is_empty() {
            return None;
        }
        vals.sort_by(f64::total_cmp);
        let p90_idx = ((vals.len() as f64 * 0.90) as usize).min(vals.len() - 1);
        Some(vals[p90_idx])
    });

    // Interchange gap — longest gap between interchange nodes in miles
    let max_rural_interchange_gap_miles = compute_interchange_gap(g, edges);

    // B1 Redundancy — find nearest parallel interstate
    let (nearest_parallel_miles, detour_penalty_miles) = find_parallel_route(g, route_id, edges);

    // BPR-estimated PTI from V/C ratio (v1.2 A3 improvement)
    // PTI_bpr = 1 + 0.15 × (V/C_peak × 1.15)^4
    // V/C_peak = p90_aadt × K_factor / (lanes_per_dir × peak_cap_pcphpl)
    let pti_bpr_estimate = p90_aadt.zip(mean_lane_count).map(|(aadt, lanes)| {
        let lanes_per_dir = (lanes / 2.0).max(1.0) as f64;
        let peak_cap = lanes_per_dir * 2_300.0; // pcph per direction at LOS E
        let k_factor = 0.09; // peak hour as fraction of daily
        let vc_peak = (aadt * k_factor) / peak_cap;
        let pti = 1.0 + 0.15 * (vc_peak * 1.15_f64).powi(4);
        pti.min(5.0).max(1.0) as f32
    });

    // v1.2 strategic dimensions from hard-coded reference data
    use crate::strategic::{
        agricultural_export_score, military_strategic_score, usmca_corridor_score,
    };
    let intl_trade_score = usmca_corridor_score(route_id);
    let military_strategic_score_val = military_strategic_score(route_id);
    let agricultural_export_score_val = agricultural_export_score(route_id);

    let attrs = CorridorAttributes {
        is_upgrade_candidate: is_upgrade,
        p90_aadt,
        mean_aadt,
        daily_capacity,
        vc_ratio_p90,
        mean_speed_limit,
        mean_lane_count,
        annual_freight_value_b: None,
        freight_value_is_hpms_proxy: false,
        mean_pct_truck,
        p90_pti,
        mean_tti,
        mean_iri,
        nearest_parallel_miles,
        detour_penalty_miles,
        betweenness_centrality,
        port_terminus_flag: false,
        nearest_top25_port_miles: None,
        border_crossing_flag: false,
        pop_within_50mi: None,
        rural_pop_within_50mi: None,
        pct_rural_in_buffer: None,
        max_rural_interchange_gap_miles,
        corridor_gdp_b: None,
        gdp_per_capita_relative: None,
        pct_pop_below_poverty: None,
        fema_sfha_miles: None,
        max_consecutive_sfha_miles: None,
        intermodal_hub_count: 0,
        dcfc_per_100mi: None,
        bridge_count: 0,
        pct_bridges_poor: None,
        mean_year_built: None,
        // v1.2 new fields
        intl_trade_score,
        pti_bpr_estimate,
        military_strategic_score: military_strategic_score_val,
        agricultural_export_score: agricultural_export_score_val,
        // v1.4 new fields — joined externally from FARS, railroad_parallels, hazard_zones
        fatal_crash_rate: None,
        rail_parallel_flag: false,
        rail_parallel_name: None,
        wildfire_risk: None,
        tornado_risk: None,
        seismic_risk: None,
    };

    Some(Corridor {
        designation: route_id_to_designation(route_id),
        termini,
        states,
        total_miles,
        edge_count,
        edges: edges.to_vec(),
        attributes: attrs,
    })
}

/// Find the westernmost and easternmost nodes of a corridor as terminus labels.
fn find_termini(g: &HighwayGraph, edges: &[EdgeIndex]) -> [String; 2] {
    let mut west_lon = f64::MAX;
    let mut east_lon = f64::MIN;
    let mut west_lat = 0.0f64;
    let mut east_lat = 0.0f64;

    for &ei in edges {
        let edge = &g.graph[ei];
        for coord in &edge.geometry.0 {
            if coord.x < west_lon {
                west_lon = coord.x;
                west_lat = coord.y;
            }
            if coord.x > east_lon {
                east_lon = coord.x;
                east_lat = coord.y;
            }
        }
    }

    if west_lon == f64::MAX {
        return ["(unknown)".into(), "(unknown)".into()];
    }

    [
        format_coord(west_lat, west_lon),
        format_coord(east_lat, east_lon),
    ]
}

fn corridor_total_miles(g: &HighwayGraph, edges: &[EdgeIndex]) -> f64 {
    let raw_total: f64 = edges.iter().map(|&ei| g.graph[ei].length_miles).sum();
    let tiger_interstate = edges.iter().all(|&ei| {
        g.graph[ei].state.is_empty() && g.graph[ei].road_class == route_data::RoadClass::Interstate
    });

    if tiger_interstate {
        raw_total / 2.0
    } else {
        raw_total
    }
}

fn format_coord(lat: f64, lon: f64) -> String {
    let ns = if lat < 0.0 { "S" } else { "N" };
    let ew = if lon < 0.0 { "W" } else { "E" };
    format!("{:.2}°{} {:.2}°{}", lat.abs(), ns, lon.abs(), ew)
}

fn infer_states_from_geometry(g: &HighwayGraph, edges: &[EdgeIndex]) -> Vec<String> {
    edges
        .iter()
        .flat_map(|&ei| g.graph[ei].geometry.0.iter())
        .filter_map(|coord| approximate_state_code(coord.y, coord.x).map(str::to_string))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn infer_edge_state(edge: &HighwayEdge) -> String {
    if !edge.state.trim().is_empty() {
        return edge.state.clone();
    }
    let mut state_counts = std::collections::BTreeMap::<&'static str, usize>::new();
    for coord in &edge.geometry.0 {
        if let Some(state) = approximate_state_code(coord.y, coord.x) {
            *state_counts.entry(state).or_default() += 1;
        }
    }
    state_counts
        .into_iter()
        .max_by(|(state_a, count_a), (state_b, count_b)| {
            count_a.cmp(count_b).then_with(|| state_b.cmp(state_a))
        })
        .map(|(state, _)| state.to_string())
        .unwrap_or_default()
}

pub fn approximate_state_code(lat: f64, lon: f64) -> Option<&'static str> {
    match (lat, lon) {
        (35.0..=42.5, -120.2..=-114.0) => Some("NV"),
        (32.0..=42.2, -124.5..=-114.0) => Some("CA"),
        (36.8..=42.2, -114.2..=-108.8) => Some("UT"),
        (40.8..=45.2, -111.2..=-104.0) => Some("WY"),
        (39.8..=43.2, -104.2..=-95.2) => Some("NE"),
        (40.2..=43.8, -96.8..=-90.0) => Some("IA"),
        (36.8..=42.6, -91.8..=-87.0) => Some("IL"),
        (37.6..=41.9, -88.2..=-84.5) => Some("IN"),
        (38.2..=42.4, -84.9..=-80.4) => Some("OH"),
        (39.4..=42.6, -80.6..=-74.6) => Some("PA"),
        (38.8..=41.5, -75.7..=-73.8) => Some("NJ"),
        (31.0..=37.2, -114.9..=-108.8) => Some("AZ"),
        (31.0..=37.2, -109.2..=-103.0) => Some("NM"),
        (36.8..=41.0, -109.2..=-102.0) => Some("CO"),
        (25.5..=36.8, -106.7..=-93.4) => Some("TX"),
        (33.5..=37.2, -103.2..=-94.2) => Some("OK"),
        (36.8..=40.2, -102.2..=-94.4) => Some("KS"),
        (35.8..=40.8, -95.9..=-89.0) => Some("MO"),
        (33.0..=36.8, -94.8..=-89.5) => Some("AR"),
        (28.8..=33.2, -94.2..=-88.6) => Some("LA"),
        (30.0..=35.2, -91.8..=-88.0) => Some("MS"),
        (30.0..=35.2, -88.6..=-84.8) => Some("AL"),
        (30.2..=35.2, -85.8..=-80.6) => Some("GA"),
        (24.0..=31.2, -87.8..=-80.0) => Some("FL"),
        (32.0..=35.4, -83.6..=-78.4) => Some("SC"),
        (33.7..=36.8, -84.4..=-75.2) => Some("NC"),
        (35.8..=39.6, -83.8..=-75.0) => Some("VA"),
        (37.0..=40.8, -82.8..=-77.4) => Some("WV"),
        (35.8..=39.4, -89.8..=-81.8) => Some("KY"),
        (34.8..=36.8, -90.4..=-81.6) => Some("TN"),
        (38.8..=39.9, -77.2..=-75.0) => Some("MD"),
        (38.4..=39.9, -75.8..=-74.8) => Some("DE"),
        (41.0..=42.2, -73.8..=-71.7) => Some("CT"),
        (41.2..=42.9, -73.6..=-69.8) => Some("MA"),
        (41.1..=42.1, -71.9..=-71.0) => Some("RI"),
        (42.6..=45.2, -73.5..=-70.6) => Some("VT"),
        (42.6..=45.4, -72.6..=-70.6) => Some("NH"),
        (43.0..=47.6, -71.2..=-66.8) => Some("ME"),
        (40.4..=45.2, -79.9..=-71.7) => Some("NY"),
        (42.4..=47.4, -92.9..=-86.2) => Some("WI"),
        (41.4..=48.4, -92.9..=-82.0) => Some("MI"),
        (43.2..=49.2, -97.5..=-89.4) => Some("MN"),
        (42.2..=45.2, -104.2..=-96.2) => Some("SD"),
        (45.0..=49.2, -104.2..=-96.2) => Some("ND"),
        (44.2..=49.2, -116.2..=-104.0) => Some("MT"),
        (41.8..=49.2, -117.4..=-111.0) => Some("ID"),
        (41.8..=46.4, -124.8..=-116.2) => Some("OR"),
        (45.4..=49.2, -124.8..=-116.8) => Some("WA"),
        (18.8..=22.4, -160.4..=-154.4) => Some("HI"),
        (51.0..=72.0, -170.0..=-129.0) => Some("AK"),
        _ => None,
    }
}

/// Longest gap (miles) between consecutive interchange nodes along the corridor.
/// Proxy for rural interchange spacing — may not be perfectly ordered.
fn compute_interchange_gap(g: &HighwayGraph, edges: &[EdgeIndex]) -> Option<f32> {
    if edges.is_empty() {
        return None;
    }
    // Collect interchange node positions along the corridor
    let mut interchange_lons: Vec<f64> = Vec::new();
    for &ei in edges {
        let endpoints = g.graph.edge_endpoints(ei)?;
        for ni in [endpoints.0, endpoints.1] {
            if g.graph[ni].is_interchange {
                interchange_lons.push(g.graph[ni].coord.x);
            }
        }
    }
    if interchange_lons.len() < 2 {
        return None;
    }
    interchange_lons.retain(|v| v.is_finite());
    if interchange_lons.len() < 2 {
        return None;
    }
    interchange_lons.sort_by(f64::total_cmp);
    interchange_lons.dedup_by(|a, b| (*a - *b).abs() < 0.01);

    // Longest gap in degrees * ~55 miles/degree longitude at ~38°N
    let max_gap_deg = interchange_lons
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0f64, f64::max);

    Some((max_gap_deg * 55.0) as f32)
}

/// Find nearest parallel interstate (different route_id, similar orientation).
/// Returns (nearest_parallel_miles, detour_penalty_miles).
/// Simple geometric approach: find the closest other interstate centroid.
fn find_parallel_route(
    g: &HighwayGraph,
    route_id: &str,
    edges: &[EdgeIndex],
) -> (Option<f64>, Option<f64>) {
    if edges.is_empty() {
        return (None, None);
    }

    // Compute centroid of this corridor
    let coords: Vec<(f64, f64)> = edges
        .iter()
        .flat_map(|&ei| {
            g.graph[ei]
                .geometry
                .0
                .iter()
                .map(|c| (c.x, c.y))
                .collect::<Vec<_>>()
        })
        .collect();

    if coords.is_empty() {
        return (None, None);
    }

    let cx = coords.iter().map(|c| c.0).sum::<f64>() / coords.len() as f64;
    let cy = coords.iter().map(|c| c.1).sum::<f64>() / coords.len() as f64;

    // For each other interstate, compute distance from its centroid to ours
    let mut nearest_dist_deg = f64::MAX;

    for (other_id, other_edges) in &g.route_index {
        if other_id == route_id || !other_id.starts_with('I') {
            continue;
        }
        // Check first and last edge midpoints
        if let Some(&first_ei) = other_edges.first() {
            let edge = &g.graph[first_ei];
            if let Some(mid) = edge.geometry.0.get(edge.geometry.0.len() / 2) {
                let dx = mid.x - cx;
                let dy = mid.y - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < nearest_dist_deg {
                    nearest_dist_deg = dist;
                }
            }
        }
    }

    if nearest_dist_deg == f64::MAX {
        return (None, None);
    }

    // Convert degrees to approx miles (at ~38°N lat)
    let nearest_miles = nearest_dist_deg * 60.0;
    // Detour penalty is approximate: 2× the lateral distance as a proxy
    let detour_penalty = nearest_miles * 2.0;

    (Some(nearest_miles), Some(detour_penalty))
}

fn route_id_to_designation(route_id: &str) -> String {
    if let Some(num) = route_id.strip_prefix('I') {
        format!("I-{num}")
    } else {
        route_id.to_string()
    }
}

// ── Stats helpers ─────────────────────────────────────────────────────────────

fn percentile_and_mean(vals: &[f64]) -> (Option<f64>, Option<f64>) {
    if vals.is_empty() {
        return (None, None);
    }
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let p90 = percentile_f64(vals, 0.90);
    (p90, Some(mean))
}

fn percentile_f64(vals: &[f64], p: f64) -> Option<f64> {
    let mut sorted: Vec<f64> = vals.iter().copied().filter(|v| v.is_finite()).collect();
    if sorted.is_empty() {
        return None;
    }
    sorted.sort_by(f64::total_cmp);
    let idx = ((sorted.len() as f64 * p) as usize).min(sorted.len() - 1);
    Some(sorted[idx])
}

fn percentile_f32(vals: &[f32], p: f64) -> Option<f32> {
    let mut sorted: Vec<f32> = vals.iter().copied().filter(|v| v.is_finite()).collect();
    if sorted.is_empty() {
        return None;
    }
    sorted.sort_by(f32::total_cmp);
    let idx = ((sorted.len() as f64 * p) as usize).min(sorted.len() - 1);
    Some(sorted[idx])
}

fn mean_f32(vals: &[f32]) -> Option<f32> {
    if vals.is_empty() {
        None
    } else {
        Some(vals.iter().sum::<f32>() / vals.len() as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::{aggregate_corridor, approximate_state_code, format_coord, infer_edge_state};
    use crate::graph::{HighwayEdge, HighwayGraph, HighwayNode};
    use geo_types::{Coord, LineString};

    #[test]
    fn format_coord_uses_correct_hemispheres() {
        assert_eq!(format_coord(40.87, -74.00), "40.87°N 74.00°W");
        assert_eq!(format_coord(-33.87, 151.21), "33.87°S 151.21°E");
    }

    #[test]
    fn tiger_interstate_corridor_miles_are_carriageway_adjusted() {
        let mut graph = HighwayGraph::new();
        let a = node(&mut graph, 0, -122.0, 37.0);
        let b = node(&mut graph, 1, -121.0, 37.0);
        let c = node(&mut graph, 2, -122.0, 37.001);
        let d = node(&mut graph, 3, -121.0, 37.001);

        let e1 = graph.graph.add_edge(
            a,
            b,
            edge(
                "I80",
                "",
                route_data::RoadClass::Interstate,
                100.0,
                -122.0,
                -121.0,
            ),
        );
        let e2 = graph.graph.add_edge(
            c,
            d,
            edge(
                "I80",
                "",
                route_data::RoadClass::Interstate,
                100.0,
                -122.0,
                -121.0,
            ),
        );
        graph.route_index.insert("I80".to_string(), vec![e1, e2]);

        let corridor = aggregate_corridor(&graph, "I80").expect("aggregate I80");

        assert_eq!(corridor.total_miles, 100.0);
        assert_eq!(corridor.termini[1], "37.00°N 121.00°W");
    }

    #[test]
    fn stateful_nhs_corridor_miles_are_not_carriageway_adjusted() {
        let mut graph = HighwayGraph::new();
        let a = node(&mut graph, 0, -90.0, 40.0);
        let b = node(&mut graph, 1, -89.0, 40.0);
        let e = graph.graph.add_edge(
            a,
            b,
            edge(
                "I80",
                "IL",
                route_data::RoadClass::Interstate,
                100.0,
                -90.0,
                -89.0,
            ),
        );
        graph.route_index.insert("I80".to_string(), vec![e]);

        let corridor = aggregate_corridor(&graph, "I80").expect("aggregate I80");

        assert_eq!(corridor.total_miles, 100.0);
        assert_eq!(corridor.states, ["IL"]);
    }

    #[test]
    fn tiger_corridor_states_can_be_inferred_from_geometry() {
        let mut graph = HighwayGraph::new();
        let ca = node(&mut graph, 0, -122.0, 37.0);
        let nv = node(&mut graph, 1, -118.0, 39.0);
        let nj = node(&mut graph, 2, -74.1, 40.8);
        let e1 = graph.graph.add_edge(
            ca,
            nv,
            edge(
                "I80",
                "",
                route_data::RoadClass::Interstate,
                100.0,
                -122.0,
                -118.0,
            ),
        );
        let e2 = graph.graph.add_edge(
            nv,
            nj,
            edge_at(
                "I80",
                "",
                route_data::RoadClass::Interstate,
                100.0,
                (-118.0, 39.0),
                (-74.1, 40.8),
            ),
        );
        graph.route_index.insert("I80".to_string(), vec![e1, e2]);

        let corridor = aggregate_corridor(&graph, "I80").expect("aggregate I80");

        assert!(corridor.states.contains(&"CA".to_string()));
        assert!(corridor.states.contains(&"NV".to_string()));
        assert!(corridor.states.contains(&"NJ".to_string()));
    }

    #[test]
    fn approx_state_code_covers_i80_endpoints() {
        assert_eq!(approximate_state_code(37.77, -122.41), Some("CA"));
        assert_eq!(approximate_state_code(40.87, -74.00), Some("NJ"));
        assert_eq!(approximate_state_code(39.74, -104.99), Some("CO"));
    }

    #[test]
    fn edge_state_infers_from_geometry_when_source_state_is_blank() {
        let edge = edge_at(
            "US287",
            "",
            route_data::RoadClass::UsHighway,
            20.0,
            (-97.4, 32.7),
            (-97.2, 32.9),
        );

        assert_eq!(infer_edge_state(&edge), "TX");

        let sourced_edge = edge_at(
            "US287",
            "OK",
            route_data::RoadClass::UsHighway,
            20.0,
            (-97.4, 32.7),
            (-97.2, 32.9),
        );

        assert_eq!(infer_edge_state(&sourced_edge), "OK");
    }

    fn node(graph: &mut HighwayGraph, id: u64, lon: f64, lat: f64) -> petgraph::graph::NodeIndex {
        graph.graph.add_node(HighwayNode {
            id,
            coord: Coord { x: lon, y: lat },
            is_interchange: false,
        })
    }

    fn edge(
        route_id: &str,
        state: &str,
        road_class: route_data::RoadClass,
        length_miles: f64,
        lon0: f64,
        lon1: f64,
    ) -> HighwayEdge {
        edge_at(
            route_id,
            state,
            road_class,
            length_miles,
            (lon0, 37.0),
            (lon1, 37.0),
        )
    }

    fn edge_at(
        route_id: &str,
        state: &str,
        road_class: route_data::RoadClass,
        length_miles: f64,
        start: (f64, f64),
        end: (f64, f64),
    ) -> HighwayEdge {
        HighwayEdge {
            id: 0,
            route_id: route_id.to_string(),
            state: state.to_string(),
            road_class,
            geometry: LineString::from(vec![start, end]),
            length_miles,
            lane_count: None,
            aadt: None,
            pct_truck: None,
            iri: None,
            tti: None,
            pti: None,
            speed_limit: None,
        }
    }
}
