use anyhow::{Context, Result};
use route_network::Corridor;
use route_score::DimensionScores;
use std::path::Path;

const VIEWBOX_W: f64 = 1600.0;
const VIEWBOX_H: f64 = 900.0;

/// Build an SVG string for a corridor map.
pub fn build_svg(
    corridor: &Corridor,
    _scores: Option<&DimensionScores>,
    _color_by: Option<&str>,
) -> Result<String> {
    // TODO: project corridor geometry from EPSG:4269 to EPSG:5070 (Albers),
    // then scale to viewbox. Render US outline from embedded basemap.
    // For now: stub SVG with corridor metadata.
    let svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {vw} {vh}\">\
         <rect width=\"{vw}\" height=\"{vh}\" fill=\"#f5f5f0\"/>\
         <text x=\"800\" y=\"450\" text-anchor=\"middle\" font-size=\"48\" fill=\"#333\">\
           {desig} \u{2014} {miles:.0} miles\
         </text>\
         <text x=\"800\" y=\"510\" text-anchor=\"middle\" font-size=\"24\" fill=\"#666\">\
           {t0} \u{2192} {t1}\
         </text>\
         </svg>",
        vw = VIEWBOX_W, vh = VIEWBOX_H,
        desig = corridor.designation,
        miles = corridor.total_miles,
        t0 = corridor.termini[0],
        t1 = corridor.termini[1],
    );
    Ok(svg)
}

/// Rasterize an SVG string to a PNG file at the given dimensions.
pub fn svg_to_png(svg: &str, output: &Path, width: u32, height: u32) -> Result<()> {
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(svg, &opt)
        .context("parsing SVG")?;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("failed to allocate pixmap {width}x{height}"))?;

    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    pixmap.save_png(output)
        .with_context(|| format!("saving PNG to {}", output.display()))?;

    Ok(())
}
