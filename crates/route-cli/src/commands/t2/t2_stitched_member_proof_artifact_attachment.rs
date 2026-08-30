//! `T2StitchedMemberProofArtifactAttachment` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_capture: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-proof-artifact-attachment");
    let capture_rows = load_t2_stitched_member_proof_source_capture(&source_capture)
        .with_context(|| format!("loading {}", source_capture.display()))?;
    let rows = t2_stitched_member_proof_artifact_attachment_rows(&capture_rows);
    write_t2_stitched_member_proof_artifact_attachment(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_proof_artifact_attachment_summary(&output, &rows);

    if gate {
        let failures =
            t2_stitched_member_proof_artifact_attachment_gate_failures(&rows, &capture_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member proof artifact attachment gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member proof artifact attachment gate failed");
        }
        println!("T2 stitched member proof artifact attachment gate: PASS");
    }

    Ok(())
}
