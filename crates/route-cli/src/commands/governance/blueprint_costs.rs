//! `BlueprintCosts` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    costs: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let packages = load_blueprint_packages(&ledger)
        .with_context(|| format!("loading Blueprint package ledger {}", ledger.display()))?;
    let rows = load_blueprint_cost_ranges(&costs)
        .with_context(|| format!("loading Blueprint cost ledger {}", costs.display()))?;
    print_blueprint_cost_ranges(&rows, blockers, details);

    if gate {
        let failures = blueprint_cost_gate_failures(&rows, &packages);
        if !failures.is_empty() {
            println!();
            println!("Blueprint cost gate: FAIL");
            println!("  {} cost rows violate range rules.", failures.len());
            for failure in failures.iter().take(12) {
                println!("  - {failure}");
            }
            anyhow::bail!("blueprint cost gate failed");
        }
        println!();
        println!("Blueprint cost gate: PASS");
    }

    Ok(())
}
