//! `T4TerminalAccessEvidenceReview` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    contact_evidence: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-evidence-review");
    let contact_rows = load_t4_terminal_contact_evidence(&contact_evidence)
        .with_context(|| format!("loading {}", contact_evidence.display()))?;
    let rows = t4_terminal_access_evidence_review_rows(&contact_rows);
    write_t4_terminal_access_evidence_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_evidence_review_summary(&output, &rows);

    if gate {
        let failures =
            t4_terminal_access_evidence_review_gate_failures(&rows, &contact_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access evidence review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access evidence review gate failed");
        }
        println!();
        println!("T4 terminal access evidence review gate: PASS");
    }
        
    Ok(())
}

