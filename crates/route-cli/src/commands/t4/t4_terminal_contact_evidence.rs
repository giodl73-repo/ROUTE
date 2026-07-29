//! `T4TerminalContactEvidence` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    terminal_columns: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-contact-evidence");
    let terminal_rows = load_t4_terminal_access_columns(&terminal_columns)
        .with_context(|| format!("loading {}", terminal_columns.display()))?;
    let rows = t4_terminal_contact_evidence_rows(&terminal_rows);
    write_t4_terminal_contact_evidence(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_contact_evidence_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_contact_evidence_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal contact evidence gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal contact evidence gate failed");
        }
        println!();
        println!("T4 terminal contact evidence gate: PASS");
    }
        
    Ok(())
}

