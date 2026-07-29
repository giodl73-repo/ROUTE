//! `T4TerminalAccessSourceAccess` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    proof_review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-source-access");
    let review_rows = load_t4_terminal_access_proof_review(&proof_review)
        .with_context(|| format!("loading {}", proof_review.display()))?;
    let rows = t4_terminal_access_source_access_rows(&review_rows);
    write_t4_terminal_access_source_access(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_source_access_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_access_source_access_gate_failures(&rows, &review_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access source access gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access source access gate failed");
        }
        println!();
        println!("T4 terminal access source access gate: PASS");
    }
        
    Ok(())
}

