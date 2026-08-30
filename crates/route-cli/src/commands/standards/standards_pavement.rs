//! `StandardsPavement` command handler extracted from main.
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

    let rows = load_pavement_standards(&ledger)
        .with_context(|| format!("loading pavement standards {}", ledger.display()))?;
    print_pavement_standards(&rows, blockers, details);

    if gate {
        let failures = pavement_standard_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Pavement standards gate: FAIL");
            println!(
                "  {} pavement standard rows lack enforceable contracts.",
                failures.len()
            );
            for failure in failures.iter().take(10) {
                println!("  - {failure}");
            }
            anyhow::bail!("pavement standards gate failed");
        }
        println!();
        println!("Pavement standards gate: PASS");
    }

    Ok(())
}
