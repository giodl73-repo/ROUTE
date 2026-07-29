//! `TierPavementFundingEvidenceAcceptedMetadataCapture` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    accepted_intake: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-accepted-metadata-capture");
    let intake_rows = load_tier_pavement_funding_evidence_accepted_intake(&accepted_intake)
        .with_context(|| format!("loading {}", accepted_intake.display()))?;
    let rows = tier_pavement_funding_evidence_accepted_metadata_capture_rows(&intake_rows);
    write_tier_pavement_funding_evidence_accepted_metadata_capture(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_accepted_metadata_capture_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_evidence_accepted_metadata_capture_gate_failures(
                &rows,
                &intake_rows,
            );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence accepted metadata-capture gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!(
                "tier pavement funding evidence accepted metadata-capture gate failed"
            );
        }
        println!();
        println!("Tier pavement funding evidence accepted metadata-capture gate: PASS");
    }
        
    Ok(())
}

