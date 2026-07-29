//! `TierPavementFundingEvidenceAcceptedIntake` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    accepted_source_access: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-accepted-intake");
    let access_rows =
        load_tier_pavement_funding_evidence_accepted_source_access(&accepted_source_access)
            .with_context(|| format!("loading {}", accepted_source_access.display()))?;
    let rows = tier_pavement_funding_evidence_accepted_intake_rows(&access_rows);
    write_tier_pavement_funding_evidence_accepted_intake(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_accepted_intake_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_funding_evidence_accepted_intake_gate_failures(
            &rows,
            &access_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence accepted intake gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence accepted intake gate failed");
        }
        println!();
        println!("Tier pavement funding evidence accepted intake gate: PASS");
    }
        
    Ok(())
}

