//! `OptimizerClaimReview` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    backlog: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route optimizer-claim-review");
            let backlog_rows = load_optimizer_residual_blocker_backlog(&backlog)
                .with_context(|| format!("loading {}", backlog.display()))?;
            let rows = optimizer_claim_review_rows(&backlog_rows);
            write_optimizer_claim_review(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_optimizer_claim_review_summary(&output, &rows);

            if gate {
                let failures = optimizer_claim_review_gate_failures(&rows, &backlog_rows);
                if !failures.is_empty() {
                    println!();
                    println!("optimizer claim review gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("optimizer claim review gate failed");
                }
                println!();
                println!("optimizer claim review gate: PASS");
            }
        
    Ok(())
}

