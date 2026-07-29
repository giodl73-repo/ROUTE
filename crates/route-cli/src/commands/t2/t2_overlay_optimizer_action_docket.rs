//! `T2OverlayOptimizerActionDocket` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    repair_delta: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-overlay-optimizer-action-docket");
    let delta_rows = load_t2_bundle_overlay_repair_delta(&repair_delta)
        .with_context(|| format!("loading {}", repair_delta.display()))?;
    let rows = t2_overlay_optimizer_action_docket_rows(&delta_rows);
    write_t2_overlay_optimizer_action_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_overlay_optimizer_action_docket_summary(&output, &rows);

    if gate {
        let failures = t2_overlay_optimizer_action_docket_gate_failures(&rows, &delta_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 overlay optimizer action docket gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 overlay optimizer action docket gate failed");
        }
        println!("T2 overlay optimizer action docket gate: PASS");
    }
        
    Ok(())
}

