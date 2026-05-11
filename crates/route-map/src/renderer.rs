use crate::projection::{AlbersUS, ViewTransform};
use anyhow::{Context, Result};
use route_network::{Corridor, HighwayGraph, T1_BACKBONE_ROUTES};
use route_score::DimensionScores;
use std::path::Path;

const VIEWBOX_W: f64 = 1600.0;
const VIEWBOX_H: f64 = 900.0;

/// Metro-style color per T1 corridor (same as megamap registry).
fn corridor_color(route_id: &str) -> &'static str {
    match route_id {
        "I5" => "#ef4444",
        "I10" => "#f97316",
        "I35" => "#10b981",
        "I40" => "#eab308",
        "I75" => "#06b6d4",
        "I80" => "#3b82f6",
        "I90" => "#8b5cf6",
        "I95" => "#f43f5e",
        // T1 urban connectors
        "I110" => "#e63946",
        "I880" => "#fb923c",
        "I84" => "#a78bfa",
        "I225" => "#34d399",
        "I2" => "#f59e0b",
        "I290" => "#60a5fa",
        "I285" => "#4ade80",
        "I4" => "#f472b6",
        // US highway upgrade candidates
        _ if route_id.starts_with("US") => "#22d3ee", // cyan
        _ if route_id.starts_with("SR") => "#84cc16", // lime
        _ => "#94a3b8",                               // default gray
    }
}

fn bg_color(route_id: &str) -> &'static str {
    if T1_BACKBONE_ROUTES.contains(&route_id) {
        corridor_color(route_id)
    } else {
        "#475569"
    }
}

pub fn build_svg(
    corridor: &Corridor,
    graph: &HighwayGraph,
    scores: Option<&DimensionScores>,
    color_by: Option<&str>,
) -> Result<String> {
    let proj = AlbersUS::new();
    let view = ViewTransform::conus(VIEWBOX_W, VIEWBOX_H);
    let highlight_id = corridor.designation.replace('-', "");

    // Pick highlight color: use corridor's metro color, or score-based if requested
    let hc = if let (Some(sc), Some(dim)) = (scores, color_by) {
        match dim {
            "a1" => score_heat(sc.a1.score),
            "b1" => score_heat(sc.b1.score),
            "d1" => score_heat(sc.d1.score),
            _ => corridor_color(&highlight_id).to_string(),
        }
    } else {
        corridor_color(&highlight_id).to_string()
    };

    let mut s = String::new();
    s += &format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {W} {H}\" width=\"{W}\" height=\"{H}\">\n<rect width=\"{W}\" height=\"{H}\" fill=\"#0d1117\"/>\n", W=VIEWBOX_W, H=VIEWBOX_H);

    // Background network
    s += "<g opacity=\"0.30\">\n";
    for ei in graph.graph.edge_indices() {
        let edge = &graph.graph[ei];
        if edge.route_id == highlight_id {
            continue;
        }
        let pts = proj_edge(edge, &proj, &view);
        if pts.len() < 2 {
            continue;
        }
        let c = bg_color(&edge.route_id);
        let w = if T1_BACKBONE_ROUTES.contains(&edge.route_id.as_str()) {
            1.2
        } else {
            0.5
        };
        s += &pline(&pts, c, w, 1.0);
    }
    s += "</g>\n";

    // Highlighted corridor — glow + solid
    s += "<g>\n";
    for &ei in &corridor.edges {
        let pts = proj_edge(&graph.graph[ei], &proj, &view);
        if pts.len() < 2 {
            continue;
        }
        s += &pline(&pts, &hc, 8.0, 0.18); // glow
        s += &pline(&pts, &hc, 3.0, 1.0); // solid
    }
    s += "</g>\n";

    // Termini dots
    for &ei in corridor
        .edges
        .first()
        .into_iter()
        .chain(corridor.edges.last().into_iter())
    {
        if let Some(c) = graph.graph[ei].geometry.0.first() {
            if c.x > -125.0 && c.x < -66.0 && c.y > 24.0 && c.y < 50.0 {
                let (px, py) = view.project_to_pixel(&proj, c.x, c.y);
                s += &format!("<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\" fill=\"{hc}\" stroke=\"#0d1117\" stroke-width=\"2\"/>\n");
            }
        }
    }

    // Info panel
    let score_str = scores
        .map(|sc| format!("{:.1}/160", sc.total()))
        .unwrap_or("—".into());
    let tier_label = match highlight_id.as_str() {
        id if T1_BACKBONE_ROUTES.contains(&id) => "T1 Primary Artery",
        id if ["I110", "I880", "I84", "I225", "I2", "I290", "I285", "I4"].contains(&id) => {
            "T1 Urban (aggregate score)"
        }
        id if id.starts_with("US") => "US Highway — Upgrade Candidate",
        _ => "Interstate Corridor",
    };

    s += &format!(
        "<rect x=\"20\" y=\"20\" width=\"360\" height=\"130\" rx=\"8\" fill=\"#0d1117\" fill-opacity=\"0.88\" stroke=\"{hc}\" stroke-width=\"1.5\"/>\n\
         <text x=\"36\" y=\"54\" font-family=\"Arial,sans-serif\" font-size=\"24\" font-weight=\"bold\" fill=\"{hc}\">{desig}</text>\n\
         <text x=\"36\" y=\"76\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#94a3b8\">{tier}</text>\n\
         <text x=\"36\" y=\"98\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#e2e8f0\">{miles:.0} mi  ·  {score}</text>\n\
         <text x=\"36\" y=\"116\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#64748b\">{t0}</text>\n\
         <text x=\"36\" y=\"134\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#64748b\">→ {t1}</text>\n",
        desig=corridor.designation, tier=tier_label, miles=corridor.total_miles,
        score=score_str, t0=corridor.termini[0], t1=corridor.termini[1],
    );

    // Legend bar
    let (bot, lcy, lty, right) = (
        VIEWBOX_H - 32.0,
        VIEWBOX_H - 14.0,
        VIEWBOX_H - 10.0,
        VIEWBOX_W - 10.0,
    );
    s += &format!(
        "<rect x=\"0\" y=\"{bot}\" width=\"{W}\" height=\"32\" fill=\"#0d1117\" fill-opacity=\"0.85\"/>\n\
         <rect x=\"16\" y=\"{sy}\" width=\"28\" height=\"7\" rx=\"2\" fill=\"{hc}\"/>\n\
         <text x=\"50\" y=\"{lty}\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#94a3b8\">{desig} (highlighted)</text>\n\
         <text x=\"{right}\" y=\"{lty}\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\" text-anchor=\"end\">ROUTE v1.4 · TIGER 2023 · HPMS 2018</text>\n",
        W=VIEWBOX_W, sy=lcy-6.0, desig=corridor.designation,
    );

    s += "</svg>";
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{Coord, LineString};
    use route_network::{Corridor, CorridorAttributes, HighwayEdge, HighwayGraph, HighwayNode};
    use std::collections::HashMap;

    fn tiny_graph() -> (HighwayGraph, Corridor) {
        let mut g = HighwayGraph::new();
        let n0 = g.graph.add_node(HighwayNode {
            id: 1,
            coord: Coord { x: -100.0, y: 40.0 },
            is_interchange: false,
        });
        let n1 = g.graph.add_node(HighwayNode {
            id: 2,
            coord: Coord { x: -99.0, y: 40.5 },
            is_interchange: false,
        });
        let edge = HighwayEdge {
            id: 1,
            route_id: "I80".into(),
            state: "NE".into(),
            road_class: route_data::RoadClass::Interstate,
            geometry: LineString::from(vec![(-100.0, 40.0), (-99.0, 40.5)]),
            length_miles: 70.0,
            lane_count: Some(2),
            aadt: Some(50_000),
            pct_truck: Some(0.2),
            iri: Some(1.0),
            tti: Some(1.1),
            pti: Some(1.2),
            speed_limit: Some(70),
        };
        let ei = g.graph.add_edge(n0, n1, edge);
        g.route_index.insert("I80".into(), vec![ei]);
        g.terminus_index.insert("I80".into(), [n0, n1]);
        g.edge_betweenness = Some(HashMap::new());

        let corridor = Corridor {
            designation: "I80".into(),
            termini: ["Teaneck, NJ".into(), "San Francisco, CA".into()],
            states: vec!["NE".into()],
            total_miles: 70.0,
            edge_count: 1,
            edges: vec![ei],
            attributes: CorridorAttributes {
                p90_aadt: Some(50_000.0),
                p90_pti: Some(1.2),
                ..Default::default()
            },
        };

        (g, corridor)
    }

    #[test]
    fn corridor_svg_uses_current_160_point_score_scale() {
        let (g, corridor) = tiny_graph();
        let scores = route_score::score_corridor(
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );

        let svg = build_svg(&corridor, &g, Some(&scores), None).expect("build svg");

        assert!(svg.contains("/160"));
        assert!(!svg.contains("/120"));
    }
}

fn score_heat(score: f64) -> String {
    let t = (score / 10.0).min(1.0);
    let r = (255.0 * t) as u8;
    let g = (200.0 * (1.0 - t)) as u8;
    format!("#{r:02x}{g:02x}40")
}

fn proj_edge(
    edge: &route_network::HighwayEdge,
    proj: &AlbersUS,
    view: &ViewTransform,
) -> Vec<(f64, f64)> {
    edge.geometry
        .0
        .iter()
        .filter(|c| c.x > -125.0 && c.x < -66.0 && c.y > 24.0 && c.y < 50.0)
        .map(|c| view.project_to_pixel(proj, c.x, c.y))
        .collect()
}

fn pline(pts: &[(f64, f64)], color: &str, width: f64, opacity: f64) -> String {
    let p: String = pts
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!("<polyline points=\"{p}\" stroke=\"{color}\" stroke-width=\"{width}\" fill=\"none\" opacity=\"{opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n")
}

pub fn svg_to_png(svg: &str, output: &Path, width: u32, height: u32) -> Result<()> {
    let mut opt = resvg::usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    let tree = resvg::usvg::Tree::from_str(svg, &opt).context("parsing SVG")?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("failed to allocate pixmap {width}x{height}"))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap
        .save_png(output)
        .with_context(|| format!("saving PNG to {}", output.display()))
}
