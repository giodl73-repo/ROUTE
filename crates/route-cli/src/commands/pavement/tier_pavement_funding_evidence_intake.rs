//! `TierPavementFundingEvidenceIntake` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_access: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-intake");
    let access_rows = load_tier_pavement_funding_evidence_source_access(&source_access)
        .with_context(|| format!("loading {}", source_access.display()))?;
    let rows = tier_pavement_funding_evidence_intake_rows(&access_rows);
    write_tier_pavement_funding_evidence_intake(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_intake_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_evidence_intake_gate_failures(&rows, &access_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence intake gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence intake gate failed");
        }
        println!();
        println!("Tier pavement funding evidence intake gate: PASS");
    }
        
    Ok(())
}

