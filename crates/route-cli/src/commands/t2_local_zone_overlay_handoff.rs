//! `T2LocalZoneOverlayHandoff` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    service_docket: PathBuf,
    zone_route_columns: PathBuf,
    zone_render_board: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-local-zone-overlay-handoff");
    let docket_rows = load_t2_service_class_repair_docket(&service_docket)
        .with_context(|| format!("loading {}", service_docket.display()))?;
    let route_rows = load_t3_zone_route_columns(&zone_route_columns)
        .with_context(|| format!("loading {}", zone_route_columns.display()))?;
    let board_rows = load_t3_zone_render_board(&zone_render_board)
        .with_context(|| format!("loading {}", zone_render_board.display()))?;
    let rows = t2_local_zone_overlay_handoff_rows(&docket_rows, &route_rows, &board_rows);
    write_t2_local_zone_overlay_handoff(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_local_zone_overlay_handoff_summary(&output, &rows);

    if gate {
        let failures = t2_local_zone_overlay_handoff_gate_failures(&rows, &docket_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 local-zone overlay handoff gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 local-zone overlay handoff gate failed");
        }
        println!();
        println!("T2 local-zone overlay handoff gate: PASS");
    }
        
    Ok(())
}

