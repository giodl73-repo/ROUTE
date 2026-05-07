/// Mega-map: all interstate tiers with metro-style color registry and labels.
use anyhow::Result;
use route_network::HighwayGraph;
use crate::projection::{AlbersUS, ViewTransform};

const W: f64 = 2400.0;
const H: f64 = 1350.0;
const T1_THRESHOLD: f64 = 21.0;
const T2_THRESHOLD: f64 = 15.0;
const T3_THRESHOLD: f64 = 9.0;

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

fn route_style(route_id: &str, score: f64) -> (String, f64, f64) {
    if is_t1_route(route_id) { (t1_color(route_id).to_string(), 3.5, 1.0) }
    else if score >= T2_THRESHOLD { ("#64748b".to_string(), 1.6, 0.70) }
    else if score >= T3_THRESHOLD { ("#475569".to_string(), 0.9, 0.55) }
    else { ("#1e293b".to_string(), 0.5, 0.45) }
}

pub fn load_tier_scores(scores_path: &std::path::Path) -> std::collections::HashMap<String, f64> {
    let mut scores = std::collections::HashMap::new();
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

fn midpoint(graph: &HighwayGraph, route_id: &str, proj: &AlbersUS, view: &ViewTransform) -> Option<(f64,f64)> {
    let edges = graph.route_edges(route_id);
    let mid = edges.get(edges.len()/2)?;
    let geom = &graph.graph[*mid].geometry;
    let c = geom.0.get(geom.0.len()/2)?;
    if c.x < -125.0 || c.x > -66.0 || c.y < 24.0 || c.y > 50.0 { return None; }
    Some(view.project_to_pixel(proj, c.x, c.y))
}

pub fn build_megamap_svg(graph: &HighwayGraph, scores: &std::collections::HashMap<String,f64>) -> Result<String> {
    let proj = AlbersUS::new();
    let view = ViewTransform::conus(W, H);
    let mut s = String::new();

    s += &format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {W} {H}\" width=\"{W}\" height=\"{H}\">\n<rect width=\"{W}\" height=\"{H}\" fill=\"#0d1117\"/>\n");

    // Painter order: T4 → T3 → T2 → T1
    for (min_s, max_s) in [(f64::NEG_INFINITY,T3_THRESHOLD),(T3_THRESHOLD,T2_THRESHOLD),(T2_THRESHOLD,T1_THRESHOLD),(T1_THRESHOLD,f64::INFINITY)] {
        let t1 = min_s >= T1_THRESHOLD;
        for ei in graph.graph.edge_indices() {
            let edge = &graph.graph[ei];
            let score = scores.get(&edge.route_id).cloned().unwrap_or(0.0);
            if score < min_s || score >= max_s { continue; }
            let pts: Vec<(f64,f64)> = edge.geometry.0.iter()
                .filter(|c| c.x > -125.0&&c.x < -66.0&&c.y > 24.0&&c.y < 50.0)
                .map(|c| view.project_to_pixel(&proj,c.x,c.y)).collect();
            if pts.len() < 2 { continue; }
            let (color,width,opacity) = route_style(&edge.route_id,score);
            let p: String = pts.iter().map(|(x,y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
            if t1 { s += &format!("<polyline points=\"{p}\" stroke=\"{color}\" stroke-width=\"9\" fill=\"none\" opacity=\"0.12\" stroke-linecap=\"round\"/>\n"); }
            s += &format!("<polyline points=\"{p}\" stroke=\"{color}\" stroke-width=\"{width}\" fill=\"none\" opacity=\"{opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n");
        }
    }

    // T1 upgrade candidates — dashed lines with candidate colors
    // US-2 (Northern Tier): white dashed — the biggest missing T1
    // I-69 alignment (US-69): orange dashed
    // I-3 alignment (proposed Savannah-Detroit): yellow dashed
    let upgrade_candidates: &[(&str, &str)] = &[
        ("US2",  "#ffffff"),   // Northern Tier — white (highest priority)
        ("US30", "#fbbf24"),   // Lincoln Hwy — amber (I-80 parallel)
        ("US69", "#fb923c"),   // I-69 corridor — orange
        ("US6",  "#a78bfa"),   // Mid-country — purple
    ];
    s += "<!-- T1 upgrade candidates (dashed) -->\n";
    for (route_id, color) in upgrade_candidates {
        for ei in graph.graph.edge_indices() {
            let edge = &graph.graph[ei];
            if edge.route_id != *route_id { continue; }
            let pts: Vec<(f64,f64)> = edge.geometry.0.iter()
                .filter(|c| c.x > -125.0&&c.x < -66.0&&c.y > 24.0&&c.y < 50.0)
                .map(|c| view.project_to_pixel(&proj,c.x,c.y)).collect();
            if pts.len() < 2 { continue; }
            let p: String = pts.iter().map(|(x,y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
            // Dashed: upgrade candidate not yet built to interstate standard
            s += &format!("<polyline points=\"{p}\" stroke=\"{color}\" stroke-width=\"2.5\" fill=\"none\" opacity=\"0.75\" stroke-linecap=\"round\" stroke-dasharray=\"8,5\"/>\n");
        }
    }

    // Labels — larger and more readable
    for (route_id, label) in T1_ROUTES {
        if let Some((lx,ly)) = midpoint(graph, route_id, &proj, &view) {
            let c = t1_color(route_id);
            // Outer glow
            s += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"64\" height=\"26\" rx=\"5\" fill=\"{c}\" fill-opacity=\"0.18\"/>\n", lx-32.0, ly-20.0);
            // Filled pill
            s += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"60\" height=\"22\" rx=\"5\" fill=\"{c}\" fill-opacity=\"0.92\"/>\n", lx-30.0, ly-18.0);
            // Text
            s += &format!("<text x=\"{lx:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"bold\" fill=\"white\" text-anchor=\"middle\">{label}</text>\n", ly-2.0);
        }
    }

    // Title
    s += "<rect x=\"20\" y=\"20\" width=\"560\" height=\"84\" rx=\"6\" fill=\"#0d1117\" fill-opacity=\"0.92\" stroke=\"#21262d\" stroke-width=\"1\"/>\n";
    s += "<text x=\"36\" y=\"50\" font-family=\"Arial,sans-serif\" font-size=\"22\" font-weight=\"bold\" fill=\"#f0f6fc\">US Interstate Arterial Map</text>\n";
    s += "<text x=\"36\" y=\"70\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#8b949e\">Centrality-adjusted tier classification  ROUTE v1.1  227 corridors</text>\n";
    s += "<text x=\"36\" y=\"88\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#6e7681\">T1 arteries in signature colors  T2/T3/T4 graded gray  TIGER 2023</text>\n";

    // Upgrade candidate labels
    let upgrade_labels: &[(&str, &str, &str)] = &[
        ("US2",  "#ffffff", "US-2 ★T1★"),
        ("US30", "#fbbf24", "US-30"),
        ("US69", "#fb923c", "US-69/I-69"),
    ];
    for (route_id, color, label) in upgrade_labels {
        if let Some((lx,ly_pt)) = midpoint(graph, route_id, &proj, &view) {
            // Dashed outline pill for upgrade candidates
            s += &format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"70\" height=\"22\" rx=\"5\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.5\" stroke-dasharray=\"4,3\" opacity=\"0.85\"/>\n", lx-35.0, ly_pt-18.0);
            s += &format!("<text x=\"{lx:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"bold\" fill=\"{color}\" text-anchor=\"middle\" opacity=\"0.9\">{label}</text>\n", ly_pt-2.0);
        }
    }

    // Legend — two rows: T1 top, tiers + upgrades bottom
    let ly = H-80.0;
    s += &format!("<rect x=\"0\" y=\"{ly}\" width=\"{W}\" height=\"80\" fill=\"#010409\"/>\n");
    // T1 corridor colors — top row
    for (i,(rid,label)) in T1_ROUTES.iter().enumerate() {
        let x = 24.0 + i as f64 * 290.0;
        let c = t1_color(rid);
        s += &format!("<rect x=\"{x:.1}\" y=\"{:.1}\" width=\"44\" height=\"10\" rx=\"2\" fill=\"{c}\"/>\n<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#e2e8f0\">{label}</text>\n",ly+14.0,x+52.0,ly+25.0);
    }
    // Tier shades + upgrade candidates — bottom row
    for (i,(c,label)) in [("#64748b","T2 Major Connectors"),("#475569","T3 Regional Feeders"),("#1e293b","T4 Local Access")].iter().enumerate() {
        let x = 24.0 + i as f64 * 340.0;
        s += &format!("<rect x=\"{x:.1}\" y=\"{:.1}\" width=\"32\" height=\"7\" rx=\"1\" fill=\"{c}\"/>\n<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#8b949e\">{label}</text>\n",ly+46.0,x+38.0,ly+54.0);
    }
    // Upgrade candidates in legend
    for (i,(c,label)) in [("#ffffff","US-2 — T1 upgrade (Northern Tier)"),("#fbbf24","US-30/I-69 — proposed T1")].iter().enumerate() {
        let x = 1050.0 + i as f64 * 570.0;
        s += &format!("<line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{c}\" stroke-width=\"2\" stroke-dasharray=\"6,4\" opacity=\"0.75\"/>\n<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"{c}\" opacity=\"0.85\">{label}</text>\n",ly+50.0,x+38.0,ly+50.0,x+44.0,ly+54.0);
    }
    s += &format!("<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#484f58\" text-anchor=\"end\">github.com/giodl73-repo/ROUTE  ·  B1 scores confirm US-2 as highest-priority T1 upgrade (B1=10.0)</text>\n",W-16.0,ly+70.0);
    s += "</svg>";
    Ok(s)
}
