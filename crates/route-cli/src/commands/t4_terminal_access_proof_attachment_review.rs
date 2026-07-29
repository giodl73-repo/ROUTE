//! `T4TerminalAccessProofAttachmentReview` command handler extracted from main.
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

            println!("route t4-terminal-access-proof-attachment-review");
            let attachment_rows =
                load_t4_terminal_access_proof_artifact_attachment(&artifact_attachment)
                    .with_context(|| format!("loading {}", artifact_attachment.display()))?;
            let rows = t4_terminal_access_proof_attachment_review_rows(&attachment_rows);
            write_t4_terminal_access_proof_attachment_review(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t4_terminal_access_proof_attachment_review_summary(&output, &rows);

            if gate {
                let failures = t4_terminal_access_proof_attachment_review_gate_failures(
                    &rows,
                    &attachment_rows,
                );
                if !failures.is_empty() {
                    println!();
                    println!("T4 terminal access proof attachment review gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T4 terminal access proof attachment review gate failed");
                }
                println!();
                println!("T4 terminal access proof attachment review gate: PASS");
            }
        
    Ok(())
}

