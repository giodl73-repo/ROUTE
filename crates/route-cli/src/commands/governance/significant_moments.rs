//! `SignificantMoments` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_significant_moments(&ledger)
        .with_context(|| format!("loading moments ledger {}", ledger.display()))?;
    print_significant_moments(&rows, blockers, details);

    if gate {
        let failures = significant_moment_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Significant moments gate: FAIL");
            println!("  {} moment rows lack complete contracts.", failures.len());
            for failure in failures.iter().take(12) {
                println!("  - {failure}");
            }
            anyhow::bail!("significant moments gate failed");
        }
        println!();
        println!("Significant moments gate: PASS");
    }

    Ok(())
}
