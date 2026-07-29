//! `T3ZoneStopPlacement` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    render_board: PathBuf,
    stop_candidates: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t3-zone-stop-placement");
            let board_rows = load_t3_zone_render_board(&render_board)
                .with_context(|| format!("loading {}", render_board.display()))?;
            let stop_file = std::fs::File::open(&stop_candidates)
                .with_context(|| format!("opening {}", stop_candidates.display()))?;
            let stop_rows = parse_stop_candidates(stop_file)
                .with_context(|| format!("parsing {}", stop_candidates.display()))?;
            let rows = t3_zone_stop_placement_rows(&board_rows, &stop_rows);
            write_t3_zone_stop_placement(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t3_zone_stop_placement_summary(&output, &rows);

            if gate {
                let failures = t3_zone_stop_placement_gate_failures(&rows, &board_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T3 zone stop placement gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T3 zone stop placement gate failed");
                }
                println!();
                println!("T3 zone stop placement gate: PASS");
            }
        
    Ok(())
}

