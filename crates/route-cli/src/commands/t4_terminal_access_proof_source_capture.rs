//! `T4TerminalAccessProofSourceCapture` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    proof_intake: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-proof-source-capture");
    let intake_rows = load_t4_terminal_access_proof_intake(&proof_intake)
        .with_context(|| format!("loading {}", proof_intake.display()))?;
    let rows = t4_terminal_access_proof_source_capture_rows(&intake_rows);
    write_t4_terminal_access_proof_source_capture(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_proof_source_capture_summary(&output, &rows);

    if gate {
        let failures =
            t4_terminal_access_proof_source_capture_gate_failures(&rows, &intake_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access proof source capture gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access proof source capture gate failed");
        }
        println!();
        println!("T4 terminal access proof source capture gate: PASS");
    }
        
    Ok(())
}

