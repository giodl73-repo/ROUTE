//! `T4TerminalContactProofSourceRegistry` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    proof_docket: PathBuf,
    accepted_sources: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t4-terminal-contact-proof-source-registry");
            let proof_rows = load_t4_terminal_contact_proof_docket(&proof_docket)
                .with_context(|| format!("loading {}", proof_docket.display()))?;
            let accepted_source_rows =
                load_t4_terminal_contact_accepted_proof_sources(&accepted_sources)
                    .with_context(|| format!("loading {}", accepted_sources.display()))?;
            let rows =
                t4_terminal_contact_proof_source_registry_rows(&proof_rows, &accepted_source_rows);
            write_t4_terminal_contact_proof_source_registry(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t4_terminal_contact_proof_source_registry_summary(&output, &rows);

            if gate {
                let failures =
                    t4_terminal_contact_proof_source_registry_gate_failures(&rows, &proof_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T4 terminal contact proof source registry gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T4 terminal contact proof source registry gate failed");
                }
                println!();
                println!("T4 terminal contact proof source registry gate: PASS");
            }
        
    Ok(())
}

