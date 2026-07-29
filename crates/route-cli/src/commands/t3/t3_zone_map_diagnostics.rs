//! `T3ZoneMapDiagnostics` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    route_columns: PathBuf,
    access_gaps: PathBuf,
    map_atlas: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-zone-map-diagnostics");
    let route_rows = load_t3_zone_route_columns(&route_columns)
        .with_context(|| format!("loading {}", route_columns.display()))?;
    let gap_rows = load_t3_t4_access_gaps(&access_gaps)
        .with_context(|| format!("loading {}", access_gaps.display()))?;
    let atlas_rows = load_map_atlas(&map_atlas)
        .with_context(|| format!("loading {}", map_atlas.display()))?;
    let rows = t3_zone_map_diagnostic_rows(&route_rows, &gap_rows, &atlas_rows);
    write_t3_zone_map_diagnostics(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_zone_map_diagnostic_summary(&output, &rows);

    if gate {
        let failures = t3_zone_map_diagnostic_gate_failures(&rows, &atlas_rows);
        if !failures.is_empty() {
            println!();
            println!("T3 zone map diagnostic gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3 zone map diagnostic gate failed");
        }
        println!();
        println!("T3 zone map diagnostic gate: PASS");
    }
        
    Ok(())
}

