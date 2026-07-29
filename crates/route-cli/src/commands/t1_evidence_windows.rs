//! `T1EvidenceWindows` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    blockers: bool,
    details: bool,
    gate_windows: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_t1_evidence_windows(&ledger)
        .with_context(|| format!("loading T1 evidence windows {}", ledger.display()))?;
    print_t1_evidence_windows(&rows, blockers, details);

    if gate_windows {
        let failures = t1_evidence_window_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T1/T1 evidence-window gate: FAIL");
            println!(
                "  {} source-window rows lack metadata or violate promotion rules.",
                failures.len()
            );
            for failure in failures.iter().take(10) {
                println!("  - {}", failure);
            }
            if failures.len() > 10 {
                println!("  ... {} more", failures.len() - 10);
            }
            anyhow::bail!("T1/T1 evidence-window gate failed");
        }
        println!();
        println!("T1/T1 evidence-window gate: PASS");
    }
        
    Ok(())
}

