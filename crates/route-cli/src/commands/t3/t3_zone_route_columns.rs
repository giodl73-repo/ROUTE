//! `T3ZoneRouteColumns` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    obligations: PathBuf,
    intake: PathBuf,
    constraint_budget: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-zone-route-columns");
    let obligation_rows = load_t3_zone_access_obligations(&obligations)
        .with_context(|| format!("loading {}", obligations.display()))?;
    let intake_rows = load_t3_t4_pressure_intake(&intake)
        .with_context(|| format!("loading {}", intake.display()))?;
    let constraint_budget_rows = load_optimizer_constraint_budget(&constraint_budget)
        .with_context(|| format!("loading {}", constraint_budget.display()))?;
    let constraint_budget_index = optimizer_constraint_budget_index(&constraint_budget_rows);
    let rows = t3_zone_route_column_rows(&obligation_rows, &intake_rows, &constraint_budget_index);
    write_t3_zone_route_columns(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_zone_route_column_summary(&output, &rows);

    if gate {
        let failures = t3_zone_route_column_gate_failures(&rows, &obligation_rows);
        if !failures.is_empty() {
            println!();
            println!("T3 zone route column gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3 zone route column gate failed");
        }
        println!();
        println!("T3 zone route column gate: PASS");
    }

    Ok(())
}
