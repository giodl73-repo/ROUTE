//! `TierPavementHpmsScopeBroadening` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    unmatched_join_review: PathBuf,
    output: PathBuf,
    functional_systems: String,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route tier-pavement-hpms-scope-broadening");
            let unmatched_join_rows =
                load_tier_pavement_unmatched_join_review(&unmatched_join_review)
                    .with_context(|| format!("loading {}", unmatched_join_review.display()))?;
            let systems = parse_hpms_functional_systems(&functional_systems)?;
            let rows = tier_pavement_hpms_scope_broadening_rows(&unmatched_join_rows, &systems);
            write_tier_pavement_hpms_scope_broadening(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_tier_pavement_hpms_scope_broadening_summary(&output, &rows);

            if gate {
                let failures = tier_pavement_hpms_scope_broadening_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("Tier pavement HPMS scope broadening gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("tier pavement HPMS scope broadening gate failed");
                }
                println!();
                println!("Tier pavement HPMS scope broadening gate: PASS");
            }
        
    Ok(())
}

