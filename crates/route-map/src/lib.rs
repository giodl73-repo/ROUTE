use anyhow::Result;
use route_network::Corridor;
use route_score::DimensionScores;
use std::path::Path;

pub mod renderer;
pub mod basemap;

/// Render a corridor map to a PNG file.
/// `color_by`: optional dimension code (e.g. "a2", "d1") for choropleth colouring.
/// Default: solid highlight colour.
pub fn render_corridor(
    corridor: &Corridor,
    scores: Option<&DimensionScores>,
    output: &Path,
    color_by: Option<&str>,
) -> Result<()> {
    let svg = renderer::build_svg(corridor, scores, color_by)?;
    renderer::svg_to_png(&svg, output, 1600, 900)
}
