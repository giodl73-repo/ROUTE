//! `T4TerminalAccessProofArtifactAcquisitionTargets` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    attachment_review: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-proof-artifact-acquisition-targets");
    let review_rows = load_t4_terminal_access_proof_attachment_review(&attachment_review)
        .with_context(|| format!("loading {}", attachment_review.display()))?;
    let rows = t4_terminal_access_proof_artifact_acquisition_target_rows(&review_rows);
    write_t4_terminal_access_proof_artifact_acquisition_targets(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_proof_artifact_acquisition_target_summary(&output, &rows);

    if gate {
        let failures =
            t4_terminal_access_proof_artifact_acquisition_target_gate_failures(&rows, &review_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access proof artifact acquisition targets gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access proof artifact acquisition targets gate failed");
        }
        println!();
        println!("T4 terminal access proof artifact acquisition targets gate: PASS");
    }

    Ok(())
}
