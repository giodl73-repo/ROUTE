//! `T4TerminalAccessProofArtifactSourceAccess` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    acquisition_targets: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-proof-artifact-source-access");
    let target_rows =
        load_t4_terminal_access_proof_artifact_acquisition_targets(&acquisition_targets)
            .with_context(|| format!("loading {}", acquisition_targets.display()))?;
    let rows = t4_terminal_access_proof_artifact_source_access_rows(&target_rows);
    write_t4_terminal_access_proof_artifact_source_access(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_proof_artifact_source_access_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_access_proof_artifact_source_access_gate_failures(
            &rows,
            &target_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access proof artifact source access gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access proof artifact source access gate failed");
        }
        println!();
        println!("T4 terminal access proof artifact source access gate: PASS");
    }
        
    Ok(())
}

