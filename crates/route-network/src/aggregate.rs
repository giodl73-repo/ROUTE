/// Build CorridorAttributes from graph edges for a single corridor.
/// Only uses data already present in the graph — no external joins here.
/// Fields requiring HPMS/NBI/Census etc. stay None until joined separately.
use crate::corridor::{Corridor, CorridorAttributes};
use crate::graph::HighwayGraph;
use petgraph::graph::EdgeIndex;

pub fn aggregate_corridor(g: &HighwayGraph, route_id: &str) -> Option<Corridor> {
    let edges = g.route_edges(route_id);
    if edges.is_empty() {
        return None;
    }

    let total_miles: f64 = edges.iter().map(|&ei| g.graph[ei].length_miles).sum();
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
        format!("{:.2}°N {:.2}°W", west_lat, -west_lon),
        format!("{:.2}°N {:.2}°E", east_lat, east_lon.abs()),
    ]
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
