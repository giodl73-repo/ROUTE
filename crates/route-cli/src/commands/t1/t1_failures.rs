//! `T1Failures` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    needs_sources: bool,
    details: bool,
    gate_evidence: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_t1_failure_ledger(&ledger)
        .with_context(|| format!("loading T1 failure ledger {}", ledger.display()))?;
    print_t1_failures(&rows, needs_sources, details);

    if gate_evidence {
        let failures = t1_failure_evidence_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T1/T1 failure evidence gate: FAIL");
            println!(
                "  {} failure rows are unlabeled or lack evidence next steps.",
                failures.len()
            );
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{} {}]: {}",
                    row.site_id, row.source_status, row.confidence, row.blocking_gap
                );
            }
            anyhow::bail!("T1/T1 failure evidence gate failed");
        }
        println!();
        println!("T1/T1 failure evidence gate: PASS");
    }

    Ok(())
}
