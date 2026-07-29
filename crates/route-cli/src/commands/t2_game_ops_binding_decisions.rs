//! `T2GameOpsBindingDecisions` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    intake: PathBuf,
    bundle_overlays: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-game-ops-binding-decisions");
            let intake_rows = load_t2_game_ops_binding_intake(&intake)
                .with_context(|| format!("loading {}", intake.display()))?;
            let overlay_rows = load_t2_bundle_overlays(&bundle_overlays)
                .with_context(|| format!("loading {}", bundle_overlays.display()))?;
            let rows = t2_game_ops_binding_decision_rows(&intake_rows, &overlay_rows);
            write_t2_game_ops_binding_decisions(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_game_ops_binding_decision_summary(&output, &rows);

            if gate {
                let failures = t2_game_ops_binding_decision_gate_failures(&rows, &intake_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T2 game/ops binding decisions gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T2 game/ops binding decisions gate failed");
                }
                println!();
                println!("T2 game/ops binding decisions gate: PASS");
            }
        
    Ok(())
}

