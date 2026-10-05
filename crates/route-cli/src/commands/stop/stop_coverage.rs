//! `StopCoverage` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    tier_table: PathBuf,
    ledger: PathBuf,
    tier: String,
    blockers: bool,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route stop-coverage --tier {tier}");
    let routes = load_tier_routes(&tier_table, &tier)
        .with_context(|| format!("loading {tier} routes from {}", tier_table.display()))?;
    let file = std::fs::File::open(&ledger)
        .with_context(|| format!("opening stop candidate ledger {}", ledger.display()))?;
    let rows = parse_stop_candidates(file)
        .with_context(|| format!("parsing stop candidate ledger {}", ledger.display()))?;
    let coverage = stop_coverage_for_routes(&rows, &routes, &tier);
    print_stop_coverage(&tier, &coverage, blockers);

    if gate {
        let failures = stop_coverage_gate_failures(&coverage);
        if !failures.is_empty() {
            println!();
            println!("stop coverage gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("stop coverage gate failed");
        }
        println!();
        println!("stop coverage gate: PASS");
    }

    Ok(())
}
