//! `T3LowerTierFeederGapBlockerRelief` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    acceptance: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t3-lower-tier-feeder-gap-blocker-relief");
            let acceptance_rows = load_t3_lower_tier_feeder_gap_policy_acceptance(&acceptance)
                .with_context(|| format!("loading {}", acceptance.display()))?;
            let rows = t3_lower_tier_feeder_gap_blocker_relief_rows(&acceptance_rows);
            write_t3_lower_tier_feeder_gap_blocker_relief(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t3_lower_tier_feeder_gap_blocker_relief_summary(&output, &rows);

            if gate {
                let failures =
                    t3_lower_tier_feeder_gap_blocker_relief_gate_failures(&rows, &acceptance_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T3 lower-tier feeder gap blocker relief gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T3 lower-tier feeder gap blocker relief gate failed");
                }
                println!();
                println!("T3 lower-tier feeder gap blocker relief gate: PASS");
            }
        
    Ok(())
}

