//! `TierPavementRepairDebtReview` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    unmatched_join_review: PathBuf,
    pavement_debt_budget: PathBuf,
    route_state_exclusions: PathBuf,
    repair_funding_acceptance: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-repair-debt-review");
    let unmatched_join_rows = load_tier_pavement_unmatched_join_review(&unmatched_join_review)
        .with_context(|| format!("loading {}", unmatched_join_review.display()))?;
    let debt_rows = load_tier_pavement_debt_budget(&pavement_debt_budget)
        .with_context(|| format!("loading {}", pavement_debt_budget.display()))?;
    let exclusion_rows = load_tier_pavement_route_state_exclusions(&route_state_exclusions)
        .with_context(|| format!("loading {}", route_state_exclusions.display()))?;
    let funding_rows = load_tier_pavement_repair_funding_acceptance(&repair_funding_acceptance)
        .with_context(|| format!("loading {}", repair_funding_acceptance.display()))?;
    let rows = tier_pavement_repair_debt_review_rows(
        &unmatched_join_rows,
        &debt_rows,
        &exclusion_rows,
        &funding_rows,
    );
    write_tier_pavement_repair_debt_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_repair_debt_review_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_repair_debt_review_gate_failures(
            &rows,
            &unmatched_join_rows,
            &exclusion_rows,
            &funding_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement repair debt review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement repair debt review gate failed");
        }
        println!();
        println!("Tier pavement repair debt review gate: PASS");
    }

    Ok(())
}
