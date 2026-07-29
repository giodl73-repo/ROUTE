//! `OptimizerResidualBlockerBacklog` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    budget: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route optimizer-residual-blocker-backlog");
    let budget_rows = load_optimizer_constraint_budget(&budget)
        .with_context(|| format!("loading {}", budget.display()))?;
    let rows = optimizer_residual_blocker_backlog_rows(&budget_rows);
    write_optimizer_residual_blocker_backlog(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_optimizer_residual_blocker_backlog_summary(&output, &rows, details);

    if gate {
        let failures =
            optimizer_residual_blocker_backlog_gate_failures(&rows, &budget_rows);
        if !failures.is_empty() {
            println!();
            println!("optimizer residual blocker backlog gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("optimizer residual blocker backlog gate failed");
        }
        println!();
        println!("optimizer residual blocker backlog gate: PASS");
    }
        
    Ok(())
}

