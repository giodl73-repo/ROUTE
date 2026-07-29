//! `TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisition` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    accepted_metadata_attachment_review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition");
            let review_rows =
                load_tier_pavement_funding_evidence_accepted_metadata_attachment_review(
                    &accepted_metadata_attachment_review,
                )
                .with_context(|| {
                    format!("loading {}", accepted_metadata_attachment_review.display())
                })?;
            let rows = tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_rows(
                &review_rows,
            );
            write_tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition(
                &output, &rows,
            )
            .with_context(|| format!("writing {}", output.display()))?;
            print_tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_summary(
                &output, &rows,
            );

            if gate {
                let failures =
                    tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_gate_failures(
                        &rows,
                        &review_rows,
                    );
                if !failures.is_empty() {
                    println!();
                    println!(
                        "Tier pavement funding evidence accepted metadata artifact acquisition gate: FAIL"
                    );
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!(
                        "tier pavement funding evidence accepted metadata artifact acquisition gate failed"
                    );
                }
                println!();
                println!(
                    "Tier pavement funding evidence accepted metadata artifact acquisition gate: PASS"
                );
            }
        
    Ok(())
}

