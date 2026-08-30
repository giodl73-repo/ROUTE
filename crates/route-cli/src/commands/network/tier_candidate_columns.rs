//! `TierCandidateColumns` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    witnesses: PathBuf,
    route_family_splits: PathBuf,
    graph_contact_validation: PathBuf,
    contact_closure: PathBuf,
    endpoint_closure: PathBuf,
    blocker_closure: PathBuf,
    pavement_debt_budget: PathBuf,
    constraint_budget: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-candidate-columns");
    let witness_rows = load_tier_contact_witnesses(&witnesses)
        .with_context(|| format!("loading {}", witnesses.display()))?;
    let route_family_rows = load_t2_route_family_splits(&route_family_splits)
        .with_context(|| format!("loading {}", route_family_splits.display()))?;
    let graph_rows = load_t2_graph_contact_validation(&graph_contact_validation)
        .with_context(|| format!("loading {}", graph_contact_validation.display()))?;
    let contact_rows = load_t2_contact_closure(&contact_closure)
        .with_context(|| format!("loading {}", contact_closure.display()))?;
    let endpoint_rows = load_t2_endpoint_closure(&endpoint_closure)
        .with_context(|| format!("loading {}", endpoint_closure.display()))?;
    let blocker_rows = load_t2_blocker_closure(&blocker_closure)
        .with_context(|| format!("loading {}", blocker_closure.display()))?;
    let pavement_debt_rows = load_tier_pavement_debt_budget(&pavement_debt_budget)
        .with_context(|| format!("loading {}", pavement_debt_budget.display()))?;
    let constraint_budget_rows = load_optimizer_constraint_budget(&constraint_budget)
        .with_context(|| format!("loading {}", constraint_budget.display()))?;
    let dispositions = t2_closure_dispositions(
        &route_family_rows,
        &graph_rows,
        &contact_rows,
        &endpoint_rows,
        &blocker_rows,
    );
    let pavement_debt_index = pavement_debt_budget_index(&pavement_debt_rows);
    let constraint_budget_index = optimizer_constraint_budget_index(&constraint_budget_rows);
    let column_rows = tier_candidate_column_rows(
        &witness_rows,
        &dispositions,
        &pavement_debt_index,
        &constraint_budget_index,
    );
    write_tier_candidate_columns(&output, &column_rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_candidate_column_summary(&output, &column_rows);

    if gate {
        let failures = tier_candidate_column_gate_failures(&column_rows);
        if !failures.is_empty() {
            println!();
            println!("tier candidate column gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier candidate column gate failed");
        }
        println!();
        println!("tier candidate column gate: PASS");
    }

    Ok(())
}
