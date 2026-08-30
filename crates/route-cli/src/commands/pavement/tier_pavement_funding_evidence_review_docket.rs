//! `TierPavementFundingEvidenceReviewDocket` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    artifact_attachment: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-review-docket");
    let attachment_rows =
        load_tier_pavement_funding_evidence_artifact_attachment(&artifact_attachment)
            .with_context(|| format!("loading {}", artifact_attachment.display()))?;
    let rows = tier_pavement_funding_evidence_review_docket_rows(&attachment_rows);
    write_tier_pavement_funding_evidence_review_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_review_docket_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_evidence_review_docket_gate_failures(&rows, &attachment_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence review-docket gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence review-docket gate failed");
        }
        println!();
        println!("Tier pavement funding evidence review-docket gate: PASS");
    }

    Ok(())
}
