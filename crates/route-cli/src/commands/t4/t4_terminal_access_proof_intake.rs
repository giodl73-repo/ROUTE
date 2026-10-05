//! `T4TerminalAccessProofIntake` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_access: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-proof-intake");
    let access_rows = load_t4_terminal_access_source_access(&source_access)
        .with_context(|| format!("loading {}", source_access.display()))?;
    let rows = t4_terminal_access_proof_intake_rows(&access_rows);
    write_t4_terminal_access_proof_intake(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_proof_intake_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_access_proof_intake_gate_failures(&rows, &access_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access proof intake gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access proof intake gate failed");
        }
        println!();
        println!("T4 terminal access proof intake gate: PASS");
    }

    Ok(())
}
