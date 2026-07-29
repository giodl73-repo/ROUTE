//! `T1DesignReview` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    tier_table: PathBuf,
    stop_candidates: PathBuf,
    sla_pairs: PathBuf,
    score_exceptions: PathBuf,
    constraint_budget: PathBuf,
    output: PathBuf,
    route_budget: usize,
    city_budget: usize,
    stop_budget: usize,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-design-review");
    let selector_rows = support::tier::t1_line_selector_rows::t1_line_selector_rows(
        &tier_table,
        &stop_candidates,
        &sla_pairs,
        &score_exceptions,
        &constraint_budget,
        route_budget,
        city_budget,
        stop_budget,
    )?;
    let rows = t1_design_review_rows(&selector_rows);
    let csv = build_t1_design_review_csv(&rows);
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&output, csv)
        .with_context(|| format!("writing {}", output.display()))?;
    let selected = rows.iter().filter(|row| row.selected).count();
    let policy_reviews = rows
        .iter()
        .filter(|row| row.design_status == "policy-review")
        .count();
    println!("  reviewed T1 candidates: {}", rows.len());
    println!("  selected T1 lines: {selected}/{route_budget}");
    println!("  policy reviews: {policy_reviews}");
    println!("  wrote review: {}", output.display());
    println!(
        "  {:<8} {:<24} {:>4} {:>5} {:<18} {:<16} Action",
        "Route", "Role", "SLA", "Stops", "Beck", "Status"
    );
    for row in &rows {
        println!(
            "  {:<8} {:<24} {:>4} {:>5} {:<18} {:<16} {}",
            row.route,
            row.design_role,
            row.promise_count,
            row.selected_stop_count,
            truncate_for_table(&row.beck_action, 18),
            row.design_status,
            row.next_design_action
        );
    }
    if gate {
        let failures = t1_design_review_gate_failures(&rows);
        if failures.is_empty() {
            println!("T1 design review gate: PASS");
        } else {
            println!("T1 design review gate: FAIL");
            for failure in failures.iter().take(10) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 design review gate failed");
        }
    }
        
    Ok(())
}

