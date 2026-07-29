//! `NationalSegmentRegistry` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    render_board: PathBuf,
    stop_placement: PathBuf,
    segment_candidates: PathBuf,
    pavement_docket: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route national-segment-registry");
    let board_rows = load_t3_zone_render_board(&render_board)
        .with_context(|| format!("loading {}", render_board.display()))?;
    let placement_rows = load_t3_zone_stop_placement(&stop_placement)
        .with_context(|| format!("loading {}", stop_placement.display()))?;
    let segment_rows = load_tier_segment_candidates(&segment_candidates)
        .with_context(|| format!("loading {}", segment_candidates.display()))?;
    let pavement_rows = load_tier_pavement_docket(&pavement_docket)
        .with_context(|| format!("loading {}", pavement_docket.display()))?;
    let rows = support::network::national_segment_registry_rows::national_segment_registry_rows(
        &board_rows,
        &placement_rows,
        &segment_rows,
        &pavement_rows,
    );
    write_national_segment_registry(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_national_segment_registry_summary(&output, &rows);

    if gate {
        let failures = national_segment_registry_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("National segment registry gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("national segment registry gate failed");
        }
        println!();
        println!("National segment registry gate: PASS");
    }
        
    Ok(())
}

