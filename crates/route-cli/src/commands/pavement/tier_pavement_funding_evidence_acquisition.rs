//! `TierPavementFundingEvidenceAcquisition` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    review_docket: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-acquisition");
    let review_rows = load_tier_pavement_funding_evidence_review_docket(&review_docket)
        .with_context(|| format!("loading {}", review_docket.display()))?;
    let rows = tier_pavement_funding_evidence_acquisition_rows(&review_rows);
    write_tier_pavement_funding_evidence_acquisition(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_acquisition_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_evidence_acquisition_gate_failures(&rows, &review_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence acquisition gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence acquisition gate failed");
        }
        println!();
        println!("Tier pavement funding evidence acquisition gate: PASS");
    }

    Ok(())
}
