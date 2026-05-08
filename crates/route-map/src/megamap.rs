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
