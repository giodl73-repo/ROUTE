//! `TierPavementFundingCommitmentReview` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    repair_funding_package: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-commitment-review");
    let package_rows =
        load_tier_pavement_repair_funding_package(&repair_funding_package)
            .with_context(|| format!("loading {}", repair_funding_package.display()))?;
    let rows = tier_pavement_funding_commitment_review_rows(&package_rows);
    write_tier_pavement_funding_commitment_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_commitment_review_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_commitment_review_gate_failures(&rows, &package_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding commitment review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding commitment review gate failed");
        }
        println!();
        println!("Tier pavement funding commitment review gate: PASS");
    }
        
    Ok(())
}

