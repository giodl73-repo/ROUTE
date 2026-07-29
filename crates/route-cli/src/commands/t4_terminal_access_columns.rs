//! `T4TerminalAccessColumns` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    intake: PathBuf,
    constraint_budget: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-access-columns");
    let intake_rows = load_t3_t4_pressure_intake(&intake)
        .with_context(|| format!("loading {}", intake.display()))?;
    let constraint_budget_rows = load_optimizer_constraint_budget(&constraint_budget)
        .with_context(|| format!("loading {}", constraint_budget.display()))?;
    let constraint_budget_index =
        optimizer_constraint_budget_index(&constraint_budget_rows);
    let rows = t4_terminal_access_column_rows(&intake_rows, &constraint_budget_index);
    write_t4_terminal_access_columns(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_access_column_summary(&output, &rows);

    if gate {
        let failures = t4_terminal_access_column_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal access column gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal access column gate failed");
        }
        println!();
        println!("T4 terminal access column gate: PASS");
    }
        
    Ok(())
}

