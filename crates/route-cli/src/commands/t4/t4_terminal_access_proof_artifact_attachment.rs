//! `T4TerminalAccessProofArtifactAttachment` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_capture: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-proof-artifact-attachment");
    let capture_rows = load_t4_terminal_access_proof_source_capture(&source_capture)
        .with_context(|| format!("loading {}", source_capture.display()))?;
    let rows = t4_terminal_access_proof_artifact_attachment_rows(&capture_rows);
    write_t4_terminal_access_proof_artifact_attachment(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_proof_artifact_attachment_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_access_proof_artifact_attachment_gate_failures(
            &rows,
            &capture_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access proof artifact attachment gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access proof artifact attachment gate failed");
        }
        println!();
        println!("T4 terminal access proof artifact attachment gate: PASS");
    }
        
    Ok(())
}

