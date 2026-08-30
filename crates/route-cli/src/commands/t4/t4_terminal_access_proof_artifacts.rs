//! `T4TerminalAccessProofArtifacts` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    proof_acquisition: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-proof-artifacts");
    let acquisition_rows = load_t4_terminal_access_proof_acquisition(&proof_acquisition)
        .with_context(|| format!("loading {}", proof_acquisition.display()))?;
    let rows = t4_terminal_access_proof_artifact_rows(&acquisition_rows);
    write_t4_terminal_access_proof_artifacts(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_proof_artifacts_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_access_proof_artifact_gate_failures(&rows, &acquisition_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access proof artifacts gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access proof artifacts gate failed");
        }
        println!();
        println!("T4 terminal access proof artifacts gate: PASS");
    }

    Ok(())
}
