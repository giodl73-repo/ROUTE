//! `T4TerminalContactProofArtifactContract` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-contact-proof-artifact-contract");
    let rows = t4_terminal_contact_proof_artifact_contract_rows();
    write_t4_terminal_contact_proof_artifact_contract(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_contact_proof_artifact_contract_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_contact_proof_artifact_contract_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal contact proof artifact contract gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal contact proof artifact contract gate failed");
        }
        println!();
        println!("T4 terminal contact proof artifact contract gate: PASS");
    }
        
    Ok(())
}

