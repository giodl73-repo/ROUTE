//! `T3ZoneRenderBoard` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    diagnostics: PathBuf,
    route_columns: PathBuf,
    access_gaps: PathBuf,
    map_atlas: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-zone-render-board");
    let diagnostic_rows = load_t3_zone_map_diagnostics(&diagnostics)
        .with_context(|| format!("loading {}", diagnostics.display()))?;
    let route_rows = load_t3_zone_route_columns(&route_columns)
        .with_context(|| format!("loading {}", route_columns.display()))?;
    let gap_rows = load_t3_t4_access_gaps(&access_gaps)
        .with_context(|| format!("loading {}", access_gaps.display()))?;
    let atlas_rows =
        load_map_atlas(&map_atlas).with_context(|| format!("loading {}", map_atlas.display()))?;
    let rows = support::tier::t3_zone_render_board_rows::t3_zone_render_board_rows(
        &diagnostic_rows,
        &route_rows,
        &gap_rows,
        &atlas_rows,
    );
    write_t3_zone_render_board(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_zone_render_board_summary(&output, &rows);

    if gate {
        let failures = t3_zone_render_board_gate_failures(&rows, &atlas_rows);
        if !failures.is_empty() {
            println!();
            println!("T3 zone render board gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3 zone render board gate failed");
        }
        println!();
        println!("T3 zone render board gate: PASS");
    }

    Ok(())
}
