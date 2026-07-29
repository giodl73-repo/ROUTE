//! `T3LowerTierFeederGapReview` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    backlog: PathBuf,
    access_gaps: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-lower-tier-feeder-gap-review");
    let backlog_rows = load_optimizer_residual_blocker_backlog(&backlog)
        .with_context(|| format!("loading {}", backlog.display()))?;
    let gap_rows = load_t3_t4_access_gaps(&access_gaps)
        .with_context(|| format!("loading {}", access_gaps.display()))?;
    let rows = t3_lower_tier_feeder_gap_review_rows(&backlog_rows, &gap_rows);
    write_t3_lower_tier_feeder_gap_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_lower_tier_feeder_gap_review_summary(&output, &rows);

    if gate {
        let failures =
            t3_lower_tier_feeder_gap_review_gate_failures(&rows, &backlog_rows, &gap_rows);
        if !failures.is_empty() {
            println!();
            println!("T3 lower-tier feeder gap review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3 lower-tier feeder gap review gate failed");
        }
        println!();
        println!("T3 lower-tier feeder gap review gate: PASS");
    }
        
    Ok(())
}

