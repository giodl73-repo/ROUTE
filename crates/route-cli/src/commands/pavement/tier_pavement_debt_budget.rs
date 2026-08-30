//! `TierPavementDebtBudget` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_gaps: PathBuf,
    route_state_exclusions: PathBuf,
    repair_funding_acceptance: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-debt-budget");
    let gap_rows = load_tier_pavement_source_gaps(&source_gaps)
        .with_context(|| format!("loading {}", source_gaps.display()))?;
    let exclusion_rows = load_tier_pavement_route_state_exclusions(&route_state_exclusions)
        .with_context(|| format!("loading {}", route_state_exclusions.display()))?;
    let funding_rows = load_tier_pavement_repair_funding_acceptance(&repair_funding_acceptance)
        .with_context(|| format!("loading {}", repair_funding_acceptance.display()))?;
    let rows =
        tier_pavement_debt_budget_rows_with_exclusions(&gap_rows, &exclusion_rows, &funding_rows);
    write_tier_pavement_debt_budget(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_debt_budget_summary(&output, &rows, details);

    if gate {
        let failures = support::pavement::tier_pavement_debt_budget_gate_failures::tier_pavement_debt_budget_gate_failures(
            &rows,
            &gap_rows,
            &exclusion_rows,
            &funding_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement debt budget gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement debt budget gate failed");
        }
        println!();
        println!("Tier pavement debt budget gate: PASS");
    }

    Ok(())
}
