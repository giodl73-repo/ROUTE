//! `T4TerminalColumbusProofAttempts` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    source_access: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t4-terminal-columbus-proof-attempts");
            let source_access_rows = load_t4_terminal_columbus_source_access(&source_access)
                .with_context(|| format!("loading {}", source_access.display()))?;
            let rows = t4_terminal_columbus_proof_attempt_rows(&source_access_rows);
            write_t4_terminal_columbus_proof_attempts(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t4_terminal_columbus_proof_attempt_summary(&output, &rows);

            if gate {
                let failures =
                    t4_terminal_columbus_proof_attempt_gate_failures(&rows, &source_access_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T4 terminal Columbus proof attempts gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T4 terminal Columbus proof attempts gate failed");
                }
                println!();
                println!("T4 terminal Columbus proof attempts gate: PASS");
            }
        
    Ok(())
}

