//! `TierPavementDowngradeExclusionDecision` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    funding_commitment_review: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-downgrade-exclusion-decision");
    let commitment_rows = load_tier_pavement_funding_commitment_review(&funding_commitment_review)
        .with_context(|| format!("loading {}", funding_commitment_review.display()))?;
    let rows = tier_pavement_downgrade_exclusion_decision_rows(&commitment_rows);
    write_tier_pavement_downgrade_exclusion_decision(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_downgrade_exclusion_decision_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_downgrade_exclusion_decision_gate_failures(&rows, &commitment_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement downgrade/exclusion decision gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement downgrade/exclusion decision gate failed");
        }
        println!();
        println!("Tier pavement downgrade/exclusion decision gate: PASS");
    }

    Ok(())
}
