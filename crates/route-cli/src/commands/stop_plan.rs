//! `StopPlan` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    route: String,
    ledger: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let route = normalise_designation(&route);
    println!("route stop-plan --route {route}");
    let file = std::fs::File::open(&ledger)
        .with_context(|| format!("opening stop candidate ledger {}", ledger.display()))?;
    let rows = parse_stop_candidates(file)
        .with_context(|| format!("parsing stop candidate ledger {}", ledger.display()))?;
    let plan = stop_plan_for_route(&rows, &route);
    print_stop_plan(&route, &plan, details);

    if gate {
        let failures = stop_plan_gate_failures(&route, &plan);
        if !failures.is_empty() {
            println!();
            println!("stop plan gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("stop plan gate failed");
        }
        println!();
        println!("stop plan gate: PASS");
    }
        
    Ok(())
}

