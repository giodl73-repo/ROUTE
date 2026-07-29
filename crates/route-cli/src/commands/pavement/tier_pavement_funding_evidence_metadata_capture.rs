//! `TierPavementFundingEvidenceMetadataCapture` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    evidence_intake: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-metadata-capture");
    let intake_rows = load_tier_pavement_funding_evidence_intake(&evidence_intake)
        .with_context(|| format!("loading {}", evidence_intake.display()))?;
    let rows = tier_pavement_funding_evidence_metadata_capture_rows(&intake_rows);
    write_tier_pavement_funding_evidence_metadata_capture(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_metadata_capture_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_funding_evidence_metadata_capture_gate_failures(
            &rows,
            &intake_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence metadata-capture gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence metadata-capture gate failed");
        }
        println!();
        println!("Tier pavement funding evidence metadata-capture gate: PASS");
    }
        
    Ok(())
}

