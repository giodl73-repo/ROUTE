//! `TierOptimize` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    all_tiers: bool,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-optimize");
    let rows = support::optimizer::optimizer_run::tier_optimizer_run_rows(all_tiers)?;
    write_tier_optimizer_runs(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_optimizer_run_summary(&output, &rows);

    if gate {
        let failures = tier_optimizer_run_gate_failures(all_tiers, &rows);
        if !failures.is_empty() {
            println!();
            println!("tier optimizer bundle gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier optimizer bundle gate failed");
        }
        println!();
        println!("tier optimizer bundle gate: PASS");
    }
        
    Ok(())
}

