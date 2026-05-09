use crate::projection::{AlbersUS, ViewTransform};
/// Mega-map: all interstate tiers with metro-style color registry, hub markers, and labels.
use anyhow::Result;
use route_network::HighwayGraph;
use serde::Deserialize;
use std::collections::HashMap;

const W: f64 = 2400.0;
const H: f64 = 1350.0;
const T1_THRESHOLD: f64 = 70.0;
const T2_THRESHOLD: f64 = 48.0;
const T3_THRESHOLD: f64 = 27.5;

// ── Tier stroke weights (Beck hierarchy) ──────────────────────────────────────
const STROKE_T1: f64 = 6.0;
const STROKE_T2: f64 = 2.5;
const STROKE_T3_T4: f64 = 1.0;

fn t1_color(route_id: &str) -> &'static str {
    match route_id {
        "I5" => "#ef4444",
        "I10" => "#f97316",
        "I35" => "#10b981",
        "I40" => "#eab308",
        "I75" => "#06b6d4",
        "I80" => "#3b82f6",
        "I90" => "#8b5cf6",
        "I95" => "#f43f5e",
        // Proposed T1 — emerald green, distinct from I-35's #10b981
        "I69" | "US69" => "#059669",
        _ => "#ffffff",
    }
}

const T1_ROUTES: &[(&str, &str)] = &[
    ("I5", "I-5"),
    ("I10", "I-10"),
    ("I35", "I-35"),
    ("I40", "I-40"),
    ("I75", "I-75"),
    ("I80", "I-80"),
    ("I90", "I-90"),
    ("I95", "I-95"),
];

fn is_t1_route(route_id: &str) -> bool {
    T1_ROUTES.iter().any(|(id, _)| *id == route_id)
}

/// Upgrade-candidate US routes rendered as dashed gold lines.
const UPGRADE_CANDIDATES: &[&str] = &["US2", "US30", "US69", "US6", "US83", "US287"];

fn is_upgrade_candidate(route_id: &str) -> bool {
    UPGRADE_CANDIDATES.contains(&route_id)
}

/// Returns (stroke_color, stroke_width, opacity, is_dashed).
/// Used by `build_t1_corridor_svg`; megamap uses per-tier inline styling.
#[allow(dead_code)]
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
                    if let Ok(s) = score.parse::<f64>() {
                        scores.insert(route.to_string(), s);
                    }
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
    m.insert("Gary/Chicago, IL", (41.60_f64, -87.35_f64));
    m.insert("Atlanta, GA", (33.75, -84.39));
    m.insert("Boston, MA", (42.36, -71.06));
    m.insert("Seattle, WA", (47.61, -122.33));
    m.insert("Sacramento, CA", (38.58, -121.49));
    m.insert("San Antonio, TX", (29.42, -98.49));
    m.insert("Jacksonville, FL", (30.33, -81.66));
    m.insert("Toledo, OH", (41.66, -83.56));
    m.insert("Richmond, VA", (37.54, -77.43));
    m.insert("Wichita, KS", (37.69, -97.34));
    m.insert("Houston, TX", (29.76, -95.37));
    m.insert("Billings, MT", (45.78, -108.50));
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
        (41.60, -87.35, "Gary/Chicago, IL".to_string(), true),
        (33.75, -84.39, "Atlanta, GA".to_string(), true),
        (42.36, -71.06, "Boston, MA".to_string(), true),
        (47.61, -122.33, "Seattle, WA".to_string(), true),
        (38.58, -121.49, "Sacramento, CA".to_string(), true),
        (29.42, -98.49, "San Antonio, TX".to_string(), true),
        (30.33, -81.66, "Jacksonville, FL".to_string(), true),
        (41.66, -83.56, "Toledo, OH".to_string(), true),
        (37.54, -77.43, "Richmond, VA".to_string(), true),
        (37.69, -97.34, "Wichita, KS".to_string(), false),
        (29.76, -95.37, "Houston, TX".to_string(), false),
        (45.78, -108.50, "Billings, MT".to_string(), false),
    ]
}

/// Load relay hubs from TOML file; returns empty vec on any error (file optional).
fn load_relay_hubs(hubs_path: &std::path::Path) -> Vec<RelayHub> {
    let Ok(text) = std::fs::read_to_string(hubs_path) else {
        return vec![];
    };
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
        if color != "#ffffff" {
            return color;
        }
    }
    "#e2e8f0" // fallback: near-white
}

fn midpoint(
    graph: &HighwayGraph,
    route_id: &str,
    proj: &AlbersUS,
    view: &ViewTransform,
) -> Option<(f64, f64)> {
    let edges = graph.route_edges(route_id);
    let mid = edges.get(edges.len() / 2)?;
    let geom = &graph.graph[*mid].geometry;
    let c = geom.0.get(geom.0.len() / 2)?;
    if c.x < -125.0 || c.x > -66.0 || c.y < 24.0 || c.y > 50.0 {
        return None;
    }
    Some(view.project_to_pixel(proj, c.x, c.y))
}

pub fn build_megamap_svg(graph: &HighwayGraph, scores: &HashMap<String, f64>) -> Result<String> {
    build_megamap_svg_with_hubs(graph, scores, None)
}

// ── State abbreviation centroids (lon, lat) ───────────────────────────────────
const STATE_LABELS: &[(f64, f64, &str)] = &[
    (-86.9, 32.7, "AL"),
    (-149.4, 64.2, "AK"),
    (-111.5, 34.3, "AZ"),
    (-92.4, 34.9, "AR"),
    (-119.4, 37.2, "CA"),
    (-105.3, 39.0, "CO"),
    (-72.6, 41.6, "CT"),
    (-75.5, 39.0, "DE"),
    (-81.7, 27.9, "FL"),
    (-83.4, 32.7, "GA"),
    (-157.8, 20.3, "HI"),
    (-114.3, 44.4, "ID"),
    (-88.9, 40.0, "IL"),
    (-86.3, 40.3, "IN"),
    (-93.5, 42.1, "IA"),
    (-98.4, 38.5, "KS"),
    (-85.3, 37.5, "KY"),
    (-91.8, 31.2, "LA"),
    (-69.4, 44.5, "ME"),
    (-76.8, 39.1, "MD"),
    (-71.5, 42.3, "MA"),
    (-84.7, 44.4, "MI"),
    (-93.6, 46.4, "MN"),
    (-89.7, 32.7, "MS"),
    (-92.6, 38.4, "MO"),
    (-109.6, 47.0, "MT"),
    (-99.8, 41.5, "NE"),
    (-116.4, 38.5, "NV"),
    (-71.6, 44.0, "NH"),
    (-74.7, 40.0, "NJ"),
    (-106.1, 34.3, "NM"),
    (-75.5, 43.0, "NY"),
    (-79.4, 35.5, "NC"),
    (-100.3, 47.5, "ND"),
    (-82.8, 40.4, "OH"),
    (-97.5, 35.6, "OK"),
    (-120.6, 44.1, "OR"),
    (-77.2, 40.9, "PA"),
    (-71.5, 41.7, "RI"),
    (-80.9, 33.8, "SC"),
    (-100.3, 44.5, "SD"),
    (-86.3, 35.9, "TN"),
    (-99.4, 31.5, "TX"),
    (-111.1, 39.4, "UT"),
    (-72.7, 44.0, "VT"),
    (-78.7, 37.9, "VA"),
    (-120.7, 47.4, "WA"),
    (-80.6, 38.7, "WV"),
    (-89.5, 44.6, "WI"),
    (-107.5, 43.0, "WY"),
    (-77.0, 38.9, "DC"),
];

// ── City anchor dots for geographic orientation ────────────────────────────────
const CITY_LABELS: &[(f64, f64, &str)] = &[
    (-87.63, 41.88, "Chicago"),
    (-74.00, 40.71, "New York"),
    (-118.24, 34.05, "Los Angeles"),
    (-95.37, 29.76, "Houston"),
    (-112.07, 33.45, "Phoenix"),
    (-75.17, 39.95, "Philadelphia"),
    (-122.33, 37.78, "San Francisco"),
    (-122.33, 47.61, "Seattle"),
    (-104.98, 39.74, "Denver"),
    (-90.20, 38.63, "St. Louis"),
    (-84.39, 33.75, "Atlanta"),
    (-80.19, 25.77, "Miami"),
    (-93.26, 44.98, "Minneapolis"),
    (-71.06, 42.36, "Boston"),
    (-97.52, 35.47, "Oklahoma City"),
    (-98.49, 29.42, "San Antonio"),
    (-81.66, 30.33, "Jacksonville"),
    (-122.68, 45.52, "Portland"),
    (-83.04, 42.33, "Detroit"),
    (-77.04, 38.91, "Washington DC"),
];

/// Collect all edge segments for a route into grouped path data.
/// Each edge produces one M…L sub-path, avoiding inter-segment seams.
/// Available for external callers; megamap builds route_paths inline via edge_indices.
#[allow(dead_code)]
fn collect_route_path(
    graph: &HighwayGraph,
    route_id: &str,
    proj: &AlbersUS,
    view: &ViewTransform,
) -> Vec<Vec<(f64, f64)>> {
    graph
        .route_edges(route_id)
        .iter()
        .map(|&ei| {
            graph.graph[ei]
                .geometry
                .0
                .iter()
                .filter(|c| c.x > -125.5 && c.x < -65.5 && c.y > 23.5 && c.y < 50.5)
                .map(|c| view.project_to_pixel(proj, c.x, c.y))
                .collect::<Vec<_>>()
        })
        .filter(|pts| pts.len() >= 2)
        .collect()
}

/// Convert a list of point-list segments into a single SVG `d` attribute
/// with individual M/L move commands — one sub-path per segment.
fn segments_to_path(segments: &[Vec<(f64, f64)>]) -> String {
    let mut d = String::new();
    for seg in segments {
        if seg.len() < 2 {
            continue;
        }
        d += &format!("M {:.1} {:.1}", seg[0].0, seg[0].1);
        for pt in &seg[1..] {
            d += &format!(" L {:.1} {:.1}", pt.0, pt.1);
        }
    }
    d
}

pub fn build_megamap_svg_with_hubs(
    graph: &HighwayGraph,
    scores: &HashMap<String, f64>,
    hubs_path: Option<&std::path::Path>,
) -> Result<String> {
    let proj = AlbersUS::new();
    let view = ViewTransform::conus(W, H);
    let coords = hub_coords();

    // Load relay hubs (optional)
    let hubs: Vec<RelayHub> = match hubs_path {
        Some(p) => load_relay_hubs(p),
        None => {
            let default = std::path::Path::new("data/relay-hubs.toml");
            load_relay_hubs(default)
        }
    };

    let mut s = String::new();

    // ── 1. Background (#0f1623 — deeper navy) ─────────────────────────────────
    s += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {W} {H}\" \
         width=\"{W}\" height=\"{H}\">\n\
         <rect width=\"{W}\" height=\"{H}\" fill=\"#0f1623\"/>\n"
    );

    // ── 2. State abbreviation labels (9px, subtle gray, no background) ────────
    s += "<!-- State abbreviation labels -->\n";
    for &(lon, lat, abbr) in STATE_LABELS {
        // skip AK and HI — outside CONUS bounds
        if lon < -125.5 || lon > -65.5 || lat < 23.5 || lat > 50.5 {
            continue;
        }
        let (px, py) = view.project_to_pixel(&proj, lon, lat);
        s += &format!(
            "<text x=\"{px:.1}\" y=\"{py:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"9\" fill=\"#3d5070\" text-anchor=\"middle\" \
             dominant-baseline=\"middle\">{abbr}</text>\n"
        );
    }

    // ── 3. Group all edges by route_id for path-based rendering ──────────────
    // Build: route_id -> Vec<Vec<(f64,f64)>> (one inner vec per edge)
    let mut route_paths: HashMap<String, Vec<Vec<(f64, f64)>>> = HashMap::new();
    for ei in graph.graph.edge_indices() {
        let edge = &graph.graph[ei];
        let pts: Vec<(f64, f64)> = edge
            .geometry
            .0
            .iter()
            .filter(|c| c.x > -125.5 && c.x < -65.5 && c.y > 23.5 && c.y < 50.5)
            .map(|c| view.project_to_pixel(&proj, c.x, c.y))
            .collect();
        if pts.len() >= 2 {
            route_paths
                .entry(edge.route_id.clone())
                .or_default()
                .push(pts);
        }
    }

    // Helper: emit a <path> for collected segments
    let emit_path = |s: &mut String,
                     segs: &[Vec<(f64, f64)>],
                     stroke: &str,
                     width: f64,
                     opacity: f64,
                     extra_attrs: &str| {
        let d = segments_to_path(segs);
        if d.is_empty() {
            return;
        }
        *s += &format!(
            "<path d=\"{d}\" stroke=\"{stroke}\" stroke-width=\"{width:.1}\" \
             fill=\"none\" opacity=\"{opacity:.2}\" stroke-linecap=\"round\" \
             stroke-linejoin=\"round\"{extra_attrs}/>\n"
        );
    };

    // ── 4. T4 routes (background noise — very faint) ──────────────────────────
    s += "<!-- T4 routes -->\n";
    for (rid, segs) in &route_paths {
        if is_upgrade_candidate(rid) || is_t1_route(rid) {
            continue;
        }
        let score = scores.get(rid.as_str()).cloned().unwrap_or(0.0);
        if score >= T3_THRESHOLD {
            continue;
        } // T3+, skip
        emit_path(&mut s, segs, "#1a2540", STROKE_T3_T4, 0.40, "");
    }

    // ── 5. T3 routes ──────────────────────────────────────────────────────────
    s += "<!-- T3 routes -->\n";
    for (rid, segs) in &route_paths {
        if is_upgrade_candidate(rid) || is_t1_route(rid) {
            continue;
        }
        let score = scores.get(rid.as_str()).cloned().unwrap_or(0.0);
        if score < T3_THRESHOLD || score >= T2_THRESHOLD {
            continue;
        }
        emit_path(&mut s, segs, "#2e4060", STROKE_T3_T4, 0.52, "");
    }

    // ── 6. T2 routes ──────────────────────────────────────────────────────────
    s += "<!-- T2 routes -->\n";
    for (rid, segs) in &route_paths {
        if is_upgrade_candidate(rid) || is_t1_route(rid) {
            continue;
        }
        let score = scores.get(rid.as_str()).cloned().unwrap_or(0.0);
        if score < T2_THRESHOLD || score >= T1_THRESHOLD {
            continue;
        }
        emit_path(&mut s, segs, "#64748b", STROKE_T2, 0.68, "");
    }

    // ── 7. Upgrade candidates — dashed gold lines ─────────────────────────────
    s += "<!-- Upgrade candidates (dashed gold #DAA520) -->\n";
    for (rid, segs) in &route_paths {
        if !is_upgrade_candidate(rid) {
            continue;
        }
        emit_path(
            &mut s,
            segs,
            "#DAA520",
            2.5,
            0.78,
            " stroke-dasharray=\"8,5\"",
        );
    }

    // ── 8. T1 routes — glow halos then strokes ────────────────────────────────
    // Pass A: glow halos (14px at 20% opacity in corridor color)
    s += "<!-- T1 glow halos -->\n";
    for (route_id, _label) in T1_ROUTES {
        let color = t1_color(route_id);
        if let Some(segs) = route_paths.get(*route_id) {
            emit_path(&mut s, segs, color, 14.0, 0.18, "");
        }
    }
    // Pass B: T1 bold strokes (6px, full opacity)
    s += "<!-- T1 corridor strokes -->\n";
    for (route_id, _label) in T1_ROUTES {
        let color = t1_color(route_id);
        if let Some(segs) = route_paths.get(*route_id) {
            emit_path(&mut s, segs, color, STROKE_T1, 1.0, "");
        }
    }

    // ── 9. City anchor dots (3px white + city name) ───────────────────────────
    s += "<!-- City anchor dots -->\n";
    for &(lon, lat, city) in CITY_LABELS {
        if lon < -125.5 || lon > -65.5 || lat < 23.5 || lat > 50.5 {
            continue;
        }
        let (px, py) = view.project_to_pixel(&proj, lon, lat);
        s += &format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3\" fill=\"white\" opacity=\"0.85\"/>\n"
        );
        s += &format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"8\" fill=\"#9ab\" text-anchor=\"middle\" opacity=\"0.75\">{city}</text>\n",
            px,
            py + 11.0
        );
    }

    // ── 10. Relay hub markers ──────────────────────────────────────────────────
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
            if lon < -125.5 || lon > -65.5 || lat < 23.5 || lat > 50.5 {
                continue;
            }
            let (px, py) = view.project_to_pixel(&proj, lon, lat);
            let confirmed = hub.status == "confirmed";
            let radius: f64 = if confirmed { 8.0 } else { 6.0 };
            let marker_color = hub_color(&hub.corridors);

            if confirmed {
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
                s += &format!(
                    "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{radius:.1}\" \
                     fill=\"none\" stroke=\"#DAA520\" stroke-width=\"2\" \
                     stroke-dasharray=\"3,2\" opacity=\"0.85\"/>\n"
                );
            }

            let city_label = hub.name.split(',').next().unwrap_or(&hub.name);
            let label_y = py + radius + 14.0;
            s += &format!(
                "<text x=\"{px:.1}\" y=\"{label_y:.1}\" \
                 font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"bold\" \
                 fill=\"#e2e8f0\" text-anchor=\"middle\" opacity=\"0.9\">{city_label}</text>\n"
            );
        }
    }

    // ── 11. T1 highway shield markers (circular, Beck-style) ─────────────────
    s += "<!-- T1 highway shield markers -->\n";
    for (route_id, label) in T1_ROUTES {
        let segs = match route_paths.get(*route_id) {
            Some(s) => s,
            None => continue,
        };
        // Use midpoint helper (based on route_edges) — falls back to None if empty
        let Some((lx, ly)) = midpoint(graph, route_id, &proj, &view) else {
            continue;
        };
        let _ = segs; // segs used implicitly for existence check via route_paths
        let c = t1_color(route_id);
        // White halo circle
        s += &format!(
            "<circle cx=\"{lx:.1}\" cy=\"{ly:.1}\" r=\"22\" \
             fill=\"white\" opacity=\"0.95\"/>\n"
        );
        // Colored fill circle
        s += &format!(
            "<circle cx=\"{lx:.1}\" cy=\"{ly:.1}\" r=\"20\" \
             fill=\"{c}\" opacity=\"1.0\"/>\n"
        );
        // Route number text (e.g. "I-80")
        s += &format!(
            "<text x=\"{lx:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"13\" font-weight=\"900\" fill=\"white\" \
             text-anchor=\"middle\" dominant-baseline=\"middle\">{label}</text>\n",
            ly + 1.0
        );
    }

    // ── 12. Upgrade candidate labels (dashed-border circles, gold) ────────────
    s += "<!-- Upgrade candidate labels -->\n";
    let upgrade_labels: &[(&str, &str)] = &[
        ("US2", "I-92"),
        ("US83", "I-29S"),
        ("US287", "I-31"),
        ("US69", "I-69"),
    ];
    for (route_id, proposed_label) in upgrade_labels {
        let Some((lx, ly)) = midpoint(graph, route_id, &proj, &view) else {
            continue;
        };
        // Dashed-border circle
        s += &format!(
            "<circle cx=\"{lx:.1}\" cy=\"{ly:.1}\" r=\"18\" \
             fill=\"#0f1623\" fill-opacity=\"0.85\" stroke=\"#DAA520\" \
             stroke-width=\"1.5\" stroke-dasharray=\"4,3\" opacity=\"0.90\"/>\n"
        );
        s += &format!(
            "<text x=\"{lx:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"10\" font-weight=\"bold\" fill=\"#DAA520\" \
             text-anchor=\"middle\" dominant-baseline=\"middle\">{proposed_label}</text>\n",
            ly + 1.0
        );
    }

    // ── 13. Legend panel (bottom-right, Beck-style) ────────────────────────────
    {
        const LX: f64 = 1950.0;
        const LY: f64 = 1050.0;
        const LW: f64 = 420.0;
        const LH: f64 = 280.0;

        // Semi-transparent panel
        s += &format!(
            "<rect x=\"{LX}\" y=\"{LY}\" width=\"{LW}\" height=\"{LH}\" rx=\"8\" \
             fill=\"#1e2d3d\" fill-opacity=\"0.92\" stroke=\"#2a3550\" stroke-width=\"1\"/>\n"
        );
        // Title
        s += &format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"14\" font-weight=\"bold\" fill=\"white\" \
             text-anchor=\"middle\">INTERSTATE 2.0 TIER MAP</text>\n",
            LX + LW / 2.0,
            LY + 24.0
        );
        // Divider
        s += &format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
             stroke=\"#2a3550\" stroke-width=\"1\"/>\n",
            LX + 12.0,
            LY + 34.0,
            LX + LW - 12.0,
            LY + 34.0
        );

        // T1 corridor color entries
        let t1_legend: &[(&str, &str, &str)] = &[
            ("I5", "#ef4444", "I-5"),
            ("I10", "#f97316", "I-10"),
            ("I35", "#10b981", "I-35"),
            ("I40", "#eab308", "I-40"),
            ("I75", "#06b6d4", "I-75"),
            ("I80", "#3b82f6", "I-80"),
            ("I90", "#8b5cf6", "I-90"),
            ("I95", "#f43f5e", "I-95"),
        ];
        let cols = 2usize;
        for (i, (_rid, color, label)) in t1_legend.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let ex = LX + 16.0 + col as f64 * (LW / 2.0 - 8.0);
            let ey = LY + 48.0 + row as f64 * 20.0;
            s += &format!(
                "<rect x=\"{ex:.1}\" y=\"{ey:.1}\" width=\"28\" height=\"10\" rx=\"2\" \
                 fill=\"{color}\"/>\n\
                 <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
                 font-size=\"11\" fill=\"#e2e8f0\">{label}</text>\n",
                ex + 34.0,
                ey + 9.0
            );
        }

        // Tier grades
        let tier_y = LY + 48.0 + (t1_legend.len() / cols) as f64 * 20.0 + 8.0;
        for (i, (color, label)) in [
            ("#64748b", "T2 Major Connectors"),
            ("#2e4060", "T3 Regional Feeders"),
            ("#1a2540", "T4 Local Access"),
        ]
        .iter()
        .enumerate()
        {
            let ey = tier_y + i as f64 * 18.0;
            s += &format!(
                "<rect x=\"{:.1}\" y=\"{ey:.1}\" width=\"28\" height=\"8\" rx=\"1\" \
                 fill=\"{color}\"/>\n\
                 <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
                 font-size=\"11\" fill=\"#8b949e\">{label}</text>\n",
                LX + 16.0,
                LX + 50.0,
                ey + 8.0
            );
        }

        // Upgrade candidate entry
        let uc_y = tier_y + 3.0 * 18.0 + 8.0;
        s += &format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
             stroke=\"#DAA520\" stroke-width=\"2\" stroke-dasharray=\"6,4\" opacity=\"0.85\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"#DAA520\">Upgrade candidate (US→I)</text>\n",
            LX + 16.0,
            uc_y + 4.0,
            LX + 44.0,
            uc_y + 4.0,
            LX + 50.0,
            uc_y + 8.0
        );

        // Relay hub entries
        let hub_y = uc_y + 22.0;
        s += &format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"#3b82f6\" \
             stroke=\"white\" stroke-width=\"1.5\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"#e2e8f0\">Relay hub (confirmed)</text>\n",
            LX + 21.0,
            hub_y,
            LX + 32.0,
            hub_y + 4.0
        );
        let hub_y2 = hub_y + 18.0;
        s += &format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"none\" \
             stroke=\"#DAA520\" stroke-width=\"1.5\" stroke-dasharray=\"3,2\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"#DAA520\">Relay hub (proposed)</text>\n",
            LX + 21.0,
            hub_y2,
            LX + 32.0,
            hub_y2 + 4.0
        );

        // Rubric version watermark inside legend
        s += &format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"9\" fill=\"#3d5070\" text-anchor=\"end\">ROUTE · rubric v1.4 · /160</text>\n",
            LX + LW - 8.0, LY + LH - 8.0
        );
    }

    // ── 14. Title panel ────────────────────────────────────────────────────────
    s += "<rect x=\"20\" y=\"20\" width=\"560\" height=\"84\" rx=\"6\" \
          fill=\"#0f1623\" fill-opacity=\"0.92\" stroke=\"#2a3550\" stroke-width=\"1\"/>\n";
    s += "<text x=\"36\" y=\"50\" font-family=\"Arial,sans-serif\" font-size=\"22\" \
          font-weight=\"bold\" fill=\"#f0f6fc\">US Interstate Arterial Map</text>\n";
    s += "<text x=\"36\" y=\"70\" font-family=\"Arial,sans-serif\" font-size=\"13\" \
          fill=\"#8b949e\">Centrality-adjusted tier classification  ROUTE v1.4  /160 scale</text>\n";
    s += "<text x=\"36\" y=\"88\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
          fill=\"#6e7681\">T1 arteries in signature colors  T2/T3/T4 graded  TIGER 2023</text>\n";

    // Watermark
    s += &format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"Arial,sans-serif\" font-size=\"10\" \
         fill=\"#3d5070\" text-anchor=\"end\">\
         github.com/giodl73-repo/ROUTE  ·  Beck schematic · Bertin semiotics</text>\n",
        W - 16.0,
        H - 8.0
    );

    s += "</svg>";
    Ok(s)
}

// ── T1 Corridor Regional Map ───────────────────────────────────────────────────

/// Tier classification for a route in the regional map context.
fn regional_tier(route_id: &str, scores: &HashMap<String, f32>) -> u8 {
    if is_t1_route(route_id) {
        return 1;
    }
    let score = scores.get(route_id).cloned().unwrap_or(0.0) as f64;
    if score >= T2_THRESHOLD {
        2
    } else if score >= T3_THRESHOLD {
        3
    } else {
        4
    }
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
            if c.x < -125.0 || c.x > -66.0 || c.y < 24.0 || c.y > 50.0 {
                continue;
            }
            if c.x < lon_min {
                lon_min = c.x;
            }
            if c.x > lon_max {
                lon_max = c.x;
            }
            if c.y < lat_min {
                lat_min = c.y;
            }
            if c.y > lat_max {
                lat_max = c.y;
            }
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
        x_min: ax_min,
        x_max: ax_max,
        y_min: ay_min,
        y_max: ay_max,
        width: CW,
        height: CH,
        padding: 60.0,
    };

    let t1_color_str = t1_color(corridor_id);
    let t1_label = T1_ROUTES
        .iter()
        .find(|(id, _)| *id == corridor_id)
        .map(|(_, label)| *label)
        .unwrap_or(corridor_id);

    // Helper: project lon/lat → pixel, returns None if outside bbox
    let to_px = |lon: f64, lat: f64| -> Option<(f64, f64)> {
        if lon < bb_lon_min || lon > bb_lon_max || lat < bb_lat_min || lat > bb_lat_max {
            return None;
        }
        Some(regional_view.project_to_pixel(&proj, lon, lat))
    };

    // Helper: collect projected points for a named route
    let route_pts = |route_id: &str| -> Vec<Vec<(f64, f64)>> {
        graph
            .route_edges(route_id)
            .iter()
            .map(|&ei| {
                graph.graph[ei]
                    .geometry
                    .0
                    .iter()
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
    let draw_route = |s: &mut String,
                      segments: Vec<Vec<(f64, f64)>>,
                      stroke: &str,
                      width: f64,
                      opacity: f64| {
        for pts in segments {
            if pts.len() < 2 {
                continue;
            }
            let p: String = pts
                .iter()
                .map(|(x, y)| format!("{x:.1},{y:.1}"))
                .collect::<Vec<_>>()
                .join(" ");
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
        if rid.as_str() == corridor_id {
            continue;
        }
        if regional_tier(rid, scores) != 4 {
            continue;
        }
        let segs = route_pts(rid);
        draw_route(&mut s, segs, "#1e293b", 0.8, 0.45);
    }

    // Pass 2: T3
    for rid in &all_route_ids {
        if rid.as_str() == corridor_id {
            continue;
        }
        if regional_tier(rid, scores) != 3 {
            continue;
        }
        let segs = route_pts(rid);
        draw_route(&mut s, segs, "#475569", 1.2, 0.55);
    }

    // Pass 3: T2 + other T1 routes (medium weight)
    for rid in &all_route_ids {
        if rid.as_str() == corridor_id {
            continue;
        }
        let tier = regional_tier(rid, scores);
        if tier > 2 {
            continue;
        } // only T1 or T2
        let segs = route_pts(rid);
        if segs.is_empty() {
            continue;
        }
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
        let pts: Vec<(f64, f64)> = graph.graph[ei]
            .geometry
            .0
            .iter()
            .filter_map(|c| to_px(c.x, c.y))
            .collect();
        if pts.len() < 2 {
            continue;
        }
        let p: String = pts
            .iter()
            .map(|(x, y)| format!("{x:.1},{y:.1}"))
            .collect::<Vec<_>>()
            .join(" ");
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
            let Some((px, py)) = to_px(lon, lat) else {
                continue;
            };
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
         fill=\"#8b949e\">T1 bold · T2/other T1 medium · T3/T4 gray  ROUTE v1.4</text>\n\
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
        ly + 11.0,
        ly + 20.0
    );
    // T3 swatch
    s += &format!(
        "<rect x=\"460\" y=\"{:.1}\" width=\"28\" height=\"5\" rx=\"1\" fill=\"#475569\"/>\n\
         <text x=\"494\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
         font-size=\"11\" fill=\"#8b949e\">T3/T4 regional</text>\n",
        ly + 12.0,
        ly + 20.0
    );
    // Watermark
    s += &format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"Arial,sans-serif\" font-size=\"10\" \
         fill=\"#484f58\" text-anchor=\"end\">ROUTE  ·  github.com/giodl73-repo</text>\n",
        CW - 16.0,
        ly + 40.0
    );

    s += "</svg>";
    Ok(s)
}
