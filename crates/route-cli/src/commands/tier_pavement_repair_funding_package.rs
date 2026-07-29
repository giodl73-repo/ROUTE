//! `TierPavementRepairFundingPackage` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    repair_disposition: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route tier-pavement-repair-funding-package");
            let disposition_rows = load_tier_pavement_repair_disposition(&repair_disposition)
                .with_context(|| format!("loading {}", repair_disposition.display()))?;
            let rows = tier_pavement_repair_funding_package_rows(&disposition_rows);
            write_tier_pavement_repair_funding_package(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_tier_pavement_repair_funding_package_summary(&output, &rows);

            if gate {
                let failures =
                    tier_pavement_repair_funding_package_gate_failures(&rows, &disposition_rows);
                if !failures.is_empty() {
                    println!();
                    println!("Tier pavement repair funding package gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("tier pavement repair funding package gate failed");
                }
                println!();
                println!("Tier pavement repair funding package gate: PASS");
            }
        
    Ok(())
}

