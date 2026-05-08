/// Mega-map: all interstate tiers with metro-style color registry, hub markers, and labels.
use anyhow::Result;
use route_network::HighwayGraph;
use crate::projection::{AlbersUS, ViewTransform};
use serde::Deserialize;
use std::collections::HashMap;

const W: f64 = 2400.0;
const H: f64 = 1350.0;
const T1_THRESHOLD: f64 = 21.0;
const T2_THRESHOLD: f64 = 15.0;
const T3_THRESHOLD: f64 = 9.0;

// ── Tier stroke weights ────────────────────────────────────────────────────────
const STROKE_T1: f64 = 4.0;
const STROKE_T2: f64 = 2.5;
const STROKE_T3_T4: f64 = 1.2;

fn t1_color(route_id: &str) -> &'static str {
    match route_id {
        "I5"  => "#ef4444", "I10" => "#f97316", "I35" => "#10b981",
        "I40" => "#eab308", "I75" => "#06b6d4", "I80" => "#3b82f6",
        "I90" => "#8b5cf6", "I95" => "#f43f5e", _     => "#ffffff",
    }
}

const T1_ROUTES: &[(&str, &str)] = &[
    ("I5","I-5"),("I10","I-10"),("I35","I-35"),("I40","I-40"),
    ("I75","I-75"),("I80","I-80"),("I90","I-90"),("I95","I-95"),
];

fn is_t1_route(route_id: &str) -> bool {
    T1_ROUTES.iter().any(|(id,_)| *id == route_id)
}

/// Upgrade-candidate US routes rendered as dashed gold lines.
const UPGRADE_CANDIDATES: &[&str] = &["US2","US30","US69","US6","US83","US287"];

fn is_upgrade_candidate(route_id: &str) -> bool {
    UPGRADE_CANDIDATES.contains(&route_id)
}

/// Returns (stroke_color, stroke_width, opacity, is_dashed).
fn route_style(route_id: &str, score: f64) -> (String, f64, f64, bool) {
    if is_t1_route(route_id) {
        (t1_color(route_id).to_string(), STROKE_T1, 1.0, false)
    } else if is_upgrade_candidate(route_id) {
        // Dashed gold regardless of score — these are upgrade candidates
        ("#DAA520".to_string(), 2.5, 0.80, true)
    } else if score >= T2_THRESHOLD {
        ("#64748b".to_string(), STROKE_T2, 0.70, false)
    } else if score >= T3_THRESHOLD {
        ("#475569".to_string(), STROKE_T3_T4, 0.55, false)
    } else {
        ("#1e293b".to_string(), STROKE_T3_T4, 0.45, false)
    }
}

pub fn load_tier_scores(scores_path: &std::path::Path) -> HashMap<String, f64> {
    let mut scores = HashMap::new();
    if let Ok(mut rdr) = csv::Reader::from_path(scores_path) {
        for result in rdr.records() {
            if let Ok(r) = result {
                if let (Some(route), Some(score)) = (r.get(0), r.get(1)) {
                    if let Ok(s) = score.parse::<f64>() { scores.insert(route.to_string(), s); }
                }
            }
        }
    }
    scores
}

// ── Relay hub types ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct RelayHub {
    name: String,
    corridors: Vec<String>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct RelayHubFile {
    hubs: Vec<RelayHub>,
}

/// Hardcoded lat/lon lookup for relay hub cities.
/// Using approximate city centroids; precise enough for map marker placement.
fn hub_coords() -> HashMap<&'static str, (f64, f64)> {
    let mut m = HashMap::new();
    // (lat, lon)
    m.insert("Gary/Chicago, IL",  (41.60_f64, -87.35_f64));
    m.insert("Atlanta, GA",        (33.75,     -84.39));
    m.insert("Boston, MA",         (42.36,     -71.06));
    m.insert("Seattle, WA",        (47.61,    -122.33));
    m.insert("Sacramento, CA",     (38.58,    -121.49));
    m.insert("San Antonio, TX",    (29.42,     -98.49));
    m.insert("Jacksonville, FL",   (30.33,     -81.66));
    m.insert("Toledo, OH",         (41.66,     -83.56));
    m.insert("Richmond, VA",       (37.54,     -77.43));
    m.insert("Wichita, KS",        (37.69,     -97.34));
    m.insert("Houston, TX",        (29.76,     -95.37));
    m.insert("Billings, MT",       (45.78,    -108.50));
    m
}

/// Public coordinate table for all 12 relay hub cities.
///
/// Returns `(lat, lon, name, is_confirmed)` for each hub.
/// Confirmed hubs are the 9 T1/T1 diamond intersections;
/// proposed hubs are the 3 missing-link candidates.
/// Callers can use this to build the `hub_coords` slice for
/// `build_t1_corridor_svg`.
pub fn t1_hub_coordinates() -> Vec<(f64, f64, String, bool)> {
    vec![
        (41.60, -87.35, "Gary/Chicago, IL".to_string(),  true),
        (33.75, -84.39, "Atlanta, GA".to_string(),        true),
        (42.36, -71.06, "Boston, MA".to_string(),         true),
        (47.61,-122.33, "Seattle, WA".to_string(),        true),
        (38.58,-121.49, "Sacramento, CA".to_string(),     true),
        (29.42, -98.49, "San Antonio, TX".to_string(),    true),
        (30.33, -81.66, "Jacksonville, FL".to_string(),   true),
        (41.66, -83.56, "Toledo, OH".to_string(),         true),
        (37.54, -77.43, "Richmond, VA".to_string(),       true),
        (37.69, -97.34, "Wichita, KS".to_string(),        false),
        (29.76, -95.37, "Houston, TX".to_string(),         false),
        (45.78,-108.50, "Billings, MT".to_string(),        false),
    ]
}

/// Load relay hubs from TOML file; returns empty vec on any error (file optional).
fn load_relay_hubs(hubs_path: &std::path::Path) -> Vec<RelayHub> {
    let Ok(text) = std::fs::read_to_string(hubs_path) else { return vec![]; };
    match toml::from_str::<RelayHubFile>(&text) {
        Ok(f) => f.hubs,
        Err(e) => {
            eprintln!("warn: relay-hubs.toml parse error: {e}");
            vec![]
        }
    }
}

/// Pick a T1 color based on the first recognized T1 corridor in the hub's corridor list.
fn hub_color(corridors: &[String]) -> &'static str {
    for c in corridors {
        // corridor strings look like "I-80", "I-90", "I-35", etc.
        let normalized = c.replace('-', "").replace(' ', "");
        // Try direct match, then strip "proposed" suffix
        let key = normalized.split('(').next().unwrap_or(&normalized).trim();
        let color = t1_color(key);
        if color != "#ffffff" { return color; }
    }
    "#e2e8f0" // fallback: near-white
}

fn midpoint(graph: &HighwayGraph, route_id: &str, proj: &AlbersUS, view: &ViewTransform) -> Option<(f64,f64)> {
    let edges = graph.route_edges(route_id);
    let mid = edges.get(edges.len()/2)?;
    let geom = &graph.graph[*mid].geometry;
    let c = geom.0.get(geom.0.len()/2)?;
    if c.x < -125.0 || c.x > -66.0 || c.y < 24.0 || c.y > 50.0 { return None; }
    Some(view.project_to_pixel(proj, c.x, c.y))
}

pub fn build_megamap_svg(
    graph: &HighwayGraph,
    scores: &HashMap<String,f64>,
) -> Result<String> {
    build_megamap_svg_with_hubs(graph, scores, None)
}

pub fn build_megamap_svg_with_hubs(
    graph: &HighwayGraph,
    scores: &HashMap<String,f64>,
    hubs_path: Option<&std::path::Path>,
) -> Result<String> {
    let proj = AlbersUS::new();
    let view = ViewTransform::conus(W, H);
    let coords = hub_coords();

    // Load relay hubs (optional)
    let hubs: Vec<RelayHub> = match hubs_path {
        Some(p) => load_relay_hubs(p),
        None => {
            // Try default location relative to cwd
            let default = std::path::Path::new("data/relay-hubs.toml");
            load_relay_hubs(default)
        }
    };

    let mut s = String::new();

    s += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {W} {H}\" \
         width=\"{W}\" height=\"{H}\">\n\
         <rect width=\"{W}\" height=\"{H}\" fill=\"#0d1117\"/>\n"
    );

    // ── Painter order: T4 → T3 → T2 → T1, then upgrade candidates on top ──────
    // Draw non-upgrade, non-T1 tiers first (T4, T3, T2), then T1, then upgrades.
    let tier_bands: &[(f64, f64)] = &[
        (f64::NEG_INFINITY, T3_THRESHOLD),
        (T3_THRESHOLD, T2_THRESHOLD),
        (T2_THRESHOLD, T1_THRESHOLD),
        (T1_THRESHOLD, f64::INFINITY),
    ];

    for &(min_s, max_s) in tier_bands {
        let t1 = min_s >= T1_THRESHOLD;
        for ei in graph.graph.edge_indices() {
            let edge = &graph.graph[ei];
            // Skip upgrade candidates — drawn in a separate pass below
            if is_upgrade_candidate(&edge.route_id) { continue; }
            let score = scores.get(&edge.route_id).cloned().unwrap_or(0.0);
            if score < min_s || score >= max_s { continue; }
            let pts: Vec<(f64,f64)> = edge.geometry.0.iter()
                .filter(|c| c.x > -125.0 && c.x < -66.0 && c.y > 24.0 && c.y < 50.0)
                .map(|c| view.project_to_pixel(&proj, c.x, c.y))
                .collect();
            if pts.len() < 2 { continue; }
            let (color, width, opacity, _dashed) = route_style(&edge.route_id, score);
            let p: String = pts.iter()
                .map(|(x,y)| format!("{x:.1},{y:.1}"))
                .collect::<Vec<_>>().join(" ");
            if t1 {
                // Glow halo under T1 lines
                s += &format!(
                    "<polyline points=\"{p}\" stroke=\"{color}\" stroke-width=\"9\" \
                     fill=\"none\" opacity=\"0.12\" stroke-linecap=\"round\"/>\n"
                );
            }
            s += &format!(
                "<polyline points=\"{p}\" stroke=\"{color}\" stroke-width=\"{width}\" \
                 fill=\"none\" opacity=\"{opacity}\" stroke-linecap=\"round\" \
                 stroke-linejoin=\"round\"/>\n"
            );
        }
    }

    // ── Upgrade candidates — dashed gold lines ────────────────────────────────
    s += "<!-- Upgrade candidates (dashed gold #DAA520) -->\n";
    for ei in graph.graph.edge_indices() {
        let edge = &graph.graph[ei];
        if !is_upgrade_candidate(&edge.route_id) { continue; }
        let pts: Vec<(f64,f64)> = edge.geometry.0.iter()
            .filter(|c| c.x > -125.0 && c.x < -66.0 && c.y > 24.0 && c.y < 50.0)
            .map(|c| view.project_to_pixel(&proj, c.x, c.y))
            .collect();
        if pts.len() < 2 { continue; }
        let p: String = pts.iter()
            .map(|(x,y)| format!("{x:.1},{y:.1}"))
            .collect::<Vec<_>>().join(" ");
        s += &format!(
            "<polyline points=\"{p}\" stroke=\"#DAA520\" stroke-width=\"2.5\" \
             fill=\"none\" opacity=\"0.80\" stroke-linecap=\"round\" \
             stroke-dasharray=\"8,5\"/>\n"
        );
    }

    // ── T1 corridor labels — 14px white text on colored pill ─────────────────
    for (route_id, label) in T1_ROUTES {
        if let Some((lx, ly)) = midpoint(graph, route_id, &proj, &view) {
            let c = t1_color(route_id);
            // Outer glow
            s += &format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"64\" height=\"26\" rx=\"5\" \
                 fill=\"{c}\" fill-opacity=\"0.18\"/>\n",
                lx - 32.0, ly - 20.0
            );
            // Filled pill
            s += &format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"60\" height=\"22\" rx=\"5\" \
                 fill=\"{c}\" fill-opacity=\"0.92\"/>\n",
                lx - 30.0, ly - 18.0
            );
            // Text — 14px per spec
            s += &format!(
                "<text x=\"{lx:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
                 font-size=\"14\" font-weight=\"bold\" fill=\"white\" \
                 text-anchor=\"middle\">{label}</text>\n",
                ly - 2.0
            );
        }
    }

    // ── Upgrade candidate labels ───────────────────────────────────────────────
    let upgrade_labels: &[(&str, &str)] = &[
        ("US2",  "US-2 ★T1★"),
        ("US30", "US-30"),
        ("US69", "US-69/I-69"),
        ("US83", "US-83"),
        ("US287","US-287"),
    ];
    for (route_id, label) in upgrade_labels {
        if let Some((lx, ly_pt)) = midpoint(graph, route_id, &proj, &view) {
            s += &format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"70\" height=\"22\" rx=\"5\" \
                 fill=\"none\" stroke=\"#DAA520\" stroke-width=\"1.5\" \
                 stroke-dasharray=\"4,3\" opacity=\"0.85\"/>\n",
                lx - 35.0, ly_pt - 18.0
            );
            s += &format!(
                "<text x=\"{lx:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
                 font-size=\"11\" font-weight=\"bold\" fill=\"#DAA520\" \
                 text-anchor=\"middle\" opacity=\"0.9\">{label}</text>\n",
                ly_pt - 2.0
            );
        }
    }

    // ── Relay hub markers ─────────────────────────────────────────────────────
    if !hubs.is_empty() {
        s += "<!-- Relay hub markers -->\n";
        for hub in &hubs {
            let (lat, lon) = match coords.get(hub.name.as_str()) {
                Some(&c) => c,
                None => {
                    eprintln!("warn: no coordinates for hub {:?}", hub.name);
                    continue;
                }
            };
            // lon = x, lat = y for geographic projection
            if lon < -125.0 || lon > -66.0 || lat < 24.0 || lat > 50.0 { continue; }
            let (px, py) = view.project_to_pixel(&proj, lon, lat);
            let confirmed = hub.status == "confirmed";
            let radius: f64 = if confirmed { 8.0 } else { 6.0 };
            let marker_color = hub_color(&hub.corridors);

            if confirmed {
                // Filled circle — confirmed hub
                // White outline ring
                s += &format!(
                    "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{:.1}\" \
                     fill=\"white\" opacity=\"0.9\"/>\n",
                    radius + 2.0
                );
                s += &format!(
                    "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{radius:.1}\" \
                     fill=\"{marker_color}\" stroke=\"white\" stroke-width=\"2\" \
                     opacity=\"0.95\"/>\n"
                );
            } else {
                // Hollow circle — proposed hub
                s += &format!(
                    "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{radius:.1}\" \
                     fill=\"none\" stroke=\"#DAA520\" stroke-width=\"2\" \
                     stroke-dasharray=\"3,2\" opacity=\"0.85\"/>\n"
                );
            }

            // City name label — 11px below marker
            // Extract short city name (before comma)
            let city_label = hub.name.split(',').next().unwrap_or(&hub.name);
            let label_y = py + radius + 14.0;
            s += &format!(
                "<text x=\"{px:.1}\" y=\"{label_y:.1}\" \
                 font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"bold\" \
                 fill=\"#e2e8f0\" text-anchor=\"middle\" opacity=\"0.9\">{city_label}</text>\n"
            );
        }
    }

    // ── Title ─────────────────────────────────────────────────────────────────
    s += "<rect x=\"20\" y=\"20\" width=\"560\" height=\"84\" rx=\"6\" \
          fill=\"#0d1117\" fill-opacity=\"0.92\" stroke=\"#21262d\" stroke-width=\"1\"/>\n";
    s += "<text x=\"36\" y=\"50\" font-family=\"Arial,sans-serif\" font-size=\"22\" \
          font-weight=\"bold\" fill=\"#f0f6fc\">US Interstate Arterial Map</text>\n";
    s += "<text x=\"36\" y=\"70\" font-family=\"Arial,sans-serif\" font-size=\"13\" \
          fill=\"#8b949e\">Centrality-adjusted tier classification  ROUTE v1.1  227 corridors</text>\n";
    s += "<text x=\"36\" y=\"88\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
          fill=\"#6e7681\">T1 arteries in signature colors  T2/T3/T4 graded gray  TIGER 2023</text>\n";

    // ── Legend ─────────────────────────────────────────────────────────────────
    let ly = H - 80.0;
    s += &format!("<rect x=\"0\" y=\"{ly}\" width=\"{W}\" height=\"80\" fill=\"#010409\"/>\n");

    // T1 corridor colors — top row
    for (i, (rid, label)) in T1_ROUTES.iter().enumerate() {
        let x = 24.0 + i as f64 * 290.0;
        let c = t1_color(rid);
        s += &format!(
            "<rect x=\"{x:.1}\" y=\"{:.1}\" width=\"44\" height=\"10\" rx=\"2\" fill=\"{c}\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"13\" fill=\"#e2e8f0\">{label}</text>\n",
            ly + 14.0, x + 52.0, ly + 25.0
        );
    }

    // Tier shades — bottom row
    for (i, (c, label)) in [
        ("#64748b", "T2 Major Connectors"),
        ("#475569", "T3 Regional Feeders"),
        ("#1e293b", "T4 Local Access"),
    ].iter().enumerate() {
        let x = 24.0 + i as f64 * 340.0;
        s += &format!(
            "<rect x=\"{x:.1}\" y=\"{:.1}\" width=\"32\" height=\"7\" rx=\"1\" fill=\"{c}\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"#8b949e\">{label}</text>\n",
            ly + 46.0, x + 38.0, ly + 54.0
        );
    }

    // Upgrade candidates in legend
    for (i, (c, label)) in [
        ("#DAA520", "Gold dashed — upgrade candidate (US-2, US-30, US-69, US-83, US-287)"),
    ].iter().enumerate() {
        let x = 1050.0 + i as f64 * 570.0;
        s += &format!(
            "<line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
             stroke=\"{c}\" stroke-width=\"2\" stroke-dasharray=\"6,4\" opacity=\"0.80\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"{c}\" opacity=\"0.85\">{label}</text>\n",
            ly + 50.0, x + 38.0, ly + 50.0, x + 44.0, ly + 54.0
        );
    }

    // Hub marker legend entry
    let hub_legend_x = 1050.0;
    let hub_legend_y2 = ly + 68.0;
    s += &format!(
        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\" fill=\"#3b82f6\" \
         stroke=\"white\" stroke-width=\"1.5\"/>\n\
         <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
         font-size=\"11\" fill=\"#e2e8f0\">● Relay hub (confirmed)</text>\n",
        hub_legend_x + 6.0, hub_legend_y2,
        hub_legend_x + 16.0, hub_legend_y2 + 4.0
    );
    s += &format!(
        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"none\" \
         stroke=\"#DAA520\" stroke-width=\"1.5\" stroke-dasharray=\"3,2\"/>\n\
         <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
         font-size=\"11\" fill=\"#DAA520\">○ Relay hub (proposed)</text>\n",
        hub_legend_x + 240.0, hub_legend_y2,
        hub_legend_x + 250.0, hub_legend_y2 + 4.0
    );

    s += &format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"Arial,sans-serif\" font-size=\"10\" \
         fill=\"#484f58\" text-anchor=\"end\">\
         github.com/giodl73-repo/ROUTE  ·  B1 scores confirm US-2 as highest-priority \
         T1 upgrade (B1=10.0)</text>\n",
        W - 16.0,
        ly + 70.0
    );
    s += "</svg>";
    Ok(s)
}

// ── T1 Corridor Regional Map ───────────────────────────────────────────────────

/// Tier classification for a route in the regional map context.
fn regional_tier(route_id: &str, scores: &HashMap<String, f32>) -> u8 {
    if is_t1_route(route_id) { return 1; }
    let score = scores.get(route_id).cloned().unwrap_or(0.0) as f64;
    if score >= T2_THRESHOLD { 2 }
    else if score >= T3_THRESHOLD { 3 }
    else { 4 }
}

/// Render a regional map centered on a specific T1 corridor.
///
/// Shows:
/// - The T1 corridor in bold with its signature color
/// - T2 corridors within the bounding box (medium weight)
/// - T3/T4 corridors in the region (light gray)
/// - Optional relay hub markers at T1/T1 intersections
pub fn build_t1_corridor_svg(
    graph: &HighwayGraph,
    corridor_id: &str,
    scores: &HashMap<String, f32>,
    hub_coords: Option<&[(f64, f64, &str)]>,
) -> Result<String> {
    const CW: f64 = 1800.0;
    const CH: f64 = 1000.0;

    let proj = AlbersUS::new();

    // ── Step 1: Compute bounding box of the T1 corridor ──────────────────────
    let t1_edges = graph.route_edges(corridor_id);
    if t1_edges.is_empty() {
        anyhow::bail!("No edges found for corridor '{corridor_id}'");
    }

    let mut lon_min = f64::MAX;
    let mut lon_max = f64::MIN;
    let mut lat_min = f64::MAX;
    let mut lat_max = f64::MIN;

    for &ei in t1_edges {
        let edge = &graph.graph[ei];
        for c in &edge.geometry.0 {
            // Skip Alaska/Hawaii outliers
            if c.x < -125.0 || c.x > -66.0 || c.y < 24.0 || c.y > 50.0 { continue; }
            if c.x < lon_min { lon_min = c.x; }
            if c.x > lon_max { lon_max = c.x; }
            if c.y < lat_min { lat_min = c.y; }
            if c.y > lat_max { lat_max = c.y; }
        }
    }

    if lon_min == f64::MAX {
        anyhow::bail!("No valid CONUS coordinates for corridor '{corridor_id}'");
    }

    // ── Step 2: Add 20% padding ───────────────────────────────────────────────
    let lon_span = (lon_max - lon_min).max(2.0);
    let lat_span = (lat_max - lat_min).max(2.0);
    let lon_pad = lon_span * 0.20;
    let lat_pad = lat_span * 0.20;

    let bb_lon_min = (lon_min - lon_pad).max(-125.0);
    let bb_lon_max = (lon_max + lon_pad).min(-66.0);
    let bb_lat_min = (lat_min - lat_pad).max(24.0);
    let bb_lat_max = (lat_max + lat_pad).min(50.0);

    // ── Step 3: Build a ViewTransform for this regional bounding box ─────────
    // Project the four corners of the bbox through Albers, then fit to canvas.
    let corners = [
        proj.project(bb_lon_min, bb_lat_max), // NW
        proj.project(bb_lon_max, bb_lat_max), // NE
        proj.project(bb_lon_min, bb_lat_min), // SW
        proj.project(bb_lon_max, bb_lat_min), // SE
    ];
    let ax_min = corners.iter().map(|c| c.0).fold(f64::MAX, f64::min);
    let ax_max = corners.iter().map(|c| c.0).fold(f64::MIN, f64::max);
    let ay_min = corners.iter().map(|c| c.1).fold(f64::MAX, f64::min);
    let ay_max = corners.iter().map(|c| c.1).fold(f64::MIN, f64::max);

    let regional_view = crate::projection::ViewTransform {
        x_min: ax_min, x_max: ax_max,
        y_min: ay_min, y_max: ay_max,
        width: CW, height: CH,
        padding: 60.0,
    };

    let t1_color_str = t1_color(corridor_id);
    let t1_label = T1_ROUTES.iter().find(|(id, _)| *id == corridor_id)
        .map(|(_, label)| *label)
        .unwrap_or(corridor_id);

    // Helper: project lon/lat → pixel, returns None if outside bbox
    let to_px = |lon: f64, lat: f64| -> Option<(f64, f64)> {
        if lon < bb_lon_min || lon > bb_lon_max || lat < bb_lat_min || lat > bb_lat_max { return None; }
        Some(regional_view.project_to_pixel(&proj, lon, lat))
    };

    // Helper: collect projected points for a named route
    let route_pts = |route_id: &str| -> Vec<Vec<(f64, f64)>> {
        graph.route_edges(route_id)
            .iter()
            .map(|&ei| {
                graph.graph[ei].geometry.0.iter()
                    .filter_map(|c| to_px(c.x, c.y))
                    .collect::<Vec<_>>()
            })
            .filter(|pts| pts.len() >= 2)
            .collect()
    };

    let mut s = String::new();
    s += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {CW} {CH}\" \
         width=\"{CW}\" height=\"{CH}\">\n\
         <rect width=\"{CW}\" height=\"{CH}\" fill=\"#0d1117\"/>\n"
    );

    // Helper: emit a polyline for all pts vectors in a route
    let draw_route = |s: &mut String, segments: Vec<Vec<(f64, f64)>>, stroke: &str, width: f64, opacity: f64| {
        for pts in segments {
            if pts.len() < 2 { continue; }
            let p: String = pts.iter().map(|(x,y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
            *s += &format!(
                "<polyline points=\"{p}\" stroke=\"{stroke}\" stroke-width=\"{width}\" \
                 fill=\"none\" opacity=\"{opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
        }
    };

    // ── Step 4: Draw all routes in region by tier (back-to-front) ────────────
    // Collect all route IDs from the graph index (already deduplicated).
    let all_route_ids = graph.route_ids();

    // Pass 1: T4 (lightest)
    for rid in &all_route_ids {
        if rid.as_str() == corridor_id { continue; }
        if regional_tier(rid, scores) != 4 { continue; }
        let segs = route_pts(rid);
        draw_route(&mut s, segs, "#1e293b", 0.8, 0.45);
    }

    // Pass 2: T3
    for rid in &all_route_ids {
        if rid.as_str() == corridor_id { continue; }
        if regional_tier(rid, scores) != 3 { continue; }
        let segs = route_pts(rid);
        draw_route(&mut s, segs, "#475569", 1.2, 0.55);
    }

    // Pass 3: T2 + other T1 routes (medium weight)
    for rid in &all_route_ids {
        if rid.as_str() == corridor_id { continue; }
        let tier = regional_tier(rid, scores);
        if tier > 2 { continue; } // only T1 or T2
        let segs = route_pts(rid);
        if segs.is_empty() { continue; }
        let (stroke, width, opacity) = if tier == 1 {
            // Other T1 routes in the region — their signature color, medium weight
            (t1_color(rid).to_string(), 2.5_f64, 0.65_f64)
        } else {
            ("#64748b".to_string(), 2.0_f64, 0.60_f64)
        };
        draw_route(&mut s, segs, &stroke, width, opacity);
    }

    // Pass 4: The T1 corridor — glow + bold stroke
    s += &format!("<!-- T1 corridor: {corridor_id} -->\n");
    for &ei in t1_edges {
        let pts: Vec<(f64, f64)> = graph.graph[ei].geometry.0.iter()
            .filter_map(|c| to_px(c.x, c.y))
            .collect();
        if pts.len() < 2 { continue; }
        let p: String = pts.iter().map(|(x,y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
        // Glow halo
        s += &format!(
            "<polyline points=\"{p}\" stroke=\"{t1_color_str}\" stroke-width=\"14\" \
             fill=\"none\" opacity=\"0.15\" stroke-linecap=\"round\"/>\n"
        );
        // Bold stroke
        s += &format!(
            "<polyline points=\"{p}\" stroke=\"{t1_color_str}\" stroke-width=\"5\" \
             fill=\"none\" opacity=\"1.0\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
        );
    }

    // ── Step 5: Hub markers ───────────────────────────────────────────────────
    if let Some(hubs) = hub_coords {
        s += "<!-- Hub markers -->\n";
        for &(lat, lon, name) in hubs {
            let Some((px, py)) = to_px(lon, lat) else { continue; };
            // Confirmed hub style (filled) — using T1 color
            s += &format!(
                "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"10\" \
                 fill=\"white\" opacity=\"0.9\"/>\n"
            );
            s += &format!(
                "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"8\" \
                 fill=\"{t1_color_str}\" stroke=\"white\" stroke-width=\"2\" \
                 opacity=\"0.95\"/>\n"
            );
            // Name label
            let label_y = py + 22.0;
            s += &format!(
                "<text x=\"{px:.1}\" y=\"{label_y:.1}\" \
                 font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"bold\" \
                 fill=\"#e2e8f0\" text-anchor=\"middle\" opacity=\"0.9\">{name}</text>\n"
            );
        }
    }

    // ── Step 6: Title panel ───────────────────────────────────────────────────
    s += &format!(
        "<rect x=\"20\" y=\"20\" width=\"420\" height=\"80\" rx=\"6\" \
         fill=\"#0d1117\" fill-opacity=\"0.92\" stroke=\"{t1_color_str}\" stroke-width=\"1.5\"/>\n\
         <text x=\"36\" y=\"54\" font-family=\"Arial,sans-serif\" font-size=\"26\" \
         font-weight=\"bold\" fill=\"{t1_color_str}\">{t1_label} Regional Map</text>\n\
         <text x=\"36\" y=\"74\" font-family=\"Arial,sans-serif\" font-size=\"12\" \
         fill=\"#8b949e\">T1 bold · T2/other T1 medium · T3/T4 gray  ROUTE v1.1</text>\n\
         <text x=\"36\" y=\"90\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
         fill=\"#6e7681\">TIGER 2023  ·  Albers Equal-Area Conic</text>\n"
    );

    // ── Step 7: Legend bar ────────────────────────────────────────────────────
    let ly = CH - 50.0;
    s += &format!("<rect x=\"0\" y=\"{ly}\" width=\"{CW}\" height=\"50\" fill=\"#010409\"/>\n");

    // T1 swatch
    s += &format!(
        "<rect x=\"20\" y=\"{:.1}\" width=\"40\" height=\"10\" rx=\"2\" fill=\"{t1_color_str}\"/>\n\
         <text x=\"66\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
         font-size=\"12\" fill=\"#e2e8f0\">{t1_label} (T1 primary)</text>\n",
        ly + 10.0, ly + 20.0
    );
    // T2 swatch
    s += &format!(
        "<rect x=\"250\" y=\"{:.1}\" width=\"32\" height=\"7\" rx=\"1\" fill=\"#64748b\"/>\n\
         <text x=\"288\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
         font-size=\"11\" fill=\"#8b949e\">T2 major connectors</text>\n",
        ly + 11.0, ly + 20.0
    );
    // T3 swatch
    s += &format!(
        "<rect x=\"460\" y=\"{:.1}\" width=\"28\" height=\"5\" rx=\"1\" fill=\"#475569\"/>\n\
         <text x=\"494\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
         font-size=\"11\" fill=\"#8b949e\">T3/T4 regional</text>\n",
        ly + 12.0, ly + 20.0
    );
    // Watermark
    s += &format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"Arial,sans-serif\" font-size=\"10\" \
         fill=\"#484f58\" text-anchor=\"end\">ROUTE  ·  github.com/giodl73-repo</text>\n",
        CW - 16.0, ly + 40.0
    );

    s += "</svg>";
    Ok(s)
}
