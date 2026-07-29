//! `T2BundleOverlayRepairTargets` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    decisions: PathBuf,
    bundle_overlays: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bundle-overlay-repair-targets");
    let decision_rows = load_t2_game_ops_binding_decisions(&decisions)
        .with_context(|| format!("loading {}", decisions.display()))?;
    let overlay_rows = load_t2_bundle_overlays(&bundle_overlays)
        .with_context(|| format!("loading {}", bundle_overlays.display()))?;
    let rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &overlay_rows);
    write_t2_bundle_overlay_repair_targets(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bundle_overlay_repair_target_summary(&output, &rows);

    if gate {
        let failures = t2_bundle_overlay_repair_target_gate_failures(&rows, &decision_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bundle overlay repair targets gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bundle overlay repair targets gate failed");
        }
        println!();
        println!("T2 bundle overlay repair targets gate: PASS");
    }
        
    Ok(())
}

