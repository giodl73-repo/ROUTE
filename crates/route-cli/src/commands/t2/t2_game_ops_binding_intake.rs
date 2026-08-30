//! `T2GameOpsBindingIntake` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, budget: PathBuf, output: PathBuf, gate: bool) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-game-ops-binding-intake");
    let budget_rows = load_optimizer_constraint_budget(&budget)
        .with_context(|| format!("loading {}", budget.display()))?;
    let rows = t2_game_ops_binding_intake_rows(&budget_rows);
    write_t2_game_ops_binding_intake(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_game_ops_binding_intake_summary(&output, &rows);

    if gate {
        let failures = t2_game_ops_binding_intake_gate_failures(&rows, &budget_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 game/ops binding intake gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 game/ops binding intake gate failed");
        }
        println!();
        println!("T2 game/ops binding intake gate: PASS");
    }

    Ok(())
}
