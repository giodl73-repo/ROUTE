//! `OptimizerConstraintBudget` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route optimizer-constraint-budget");
    let ledger_rows = load_optimizer_constraint_ledger(&ledger)
        .with_context(|| format!("loading {}", ledger.display()))?;
    let rows = optimizer_constraint_budget_rows(&ledger_rows);
    write_optimizer_constraint_budget(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_optimizer_constraint_budget_summary(&output, &rows, details);

    if gate {
        let failures = optimizer_constraint_budget_gate_failures(&rows, &ledger_rows);
        if !failures.is_empty() {
            println!();
            println!("optimizer constraint budget gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("optimizer constraint budget gate failed");
        }
        println!();
        println!("optimizer constraint budget gate: PASS");
    }
        
    Ok(())
}

