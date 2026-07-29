//! `T2StitchedMemberProofReviewDocket` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    artifact_attachment: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-proof-review-docket");
    let attachment_rows =
        load_t2_stitched_member_proof_artifact_attachment(&artifact_attachment)
            .with_context(|| format!("loading {}", artifact_attachment.display()))?;
    let rows = t2_stitched_member_proof_review_docket_rows(&attachment_rows);
    write_t2_stitched_member_proof_review_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_proof_review_docket_summary(&output, &rows);

    if gate {
        let failures =
            t2_stitched_member_proof_review_docket_gate_failures(&rows, &attachment_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member proof review docket gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member proof review docket gate failed");
        }
        println!("T2 stitched member proof review docket gate: PASS");
    }
        
    Ok(())
}

