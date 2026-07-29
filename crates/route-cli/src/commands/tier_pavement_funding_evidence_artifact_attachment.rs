//! `TierPavementFundingEvidenceArtifactAttachment` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    source_capture: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-artifact-attachment");
    let capture_rows = load_tier_pavement_funding_evidence_source_capture(&source_capture)
        .with_context(|| format!("loading {}", source_capture.display()))?;
    let rows = tier_pavement_funding_evidence_artifact_attachment_rows(&capture_rows);
    write_tier_pavement_funding_evidence_artifact_attachment(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_artifact_attachment_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_funding_evidence_artifact_attachment_gate_failures(
            &rows,
            &capture_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence artifact-attachment gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence artifact-attachment gate failed");
        }
        println!();
        println!("Tier pavement funding evidence artifact-attachment gate: PASS");
    }
        
    Ok(())
}

