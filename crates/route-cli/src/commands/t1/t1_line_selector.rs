//! `T1LineSelector` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    tier_table: PathBuf,
    stop_candidates: PathBuf,
    sla_pairs: PathBuf,
    score_exceptions: PathBuf,
    constraint_budget: PathBuf,
    output: PathBuf,
    route_budget: usize,
    city_budget: usize,
    stop_budget: usize,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-line-selector");
    let rows = support::tier::t1_line_selector_rows::t1_line_selector_rows(
        &tier_table,
        &stop_candidates,
        &sla_pairs,
        &score_exceptions,
        &constraint_budget,
        route_budget,
        city_budget,
        stop_budget,
    )?;
    let csv = build_t1_line_selector_csv(&rows);
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&output, csv).with_context(|| format!("writing {}", output.display()))?;
    let selected = rows.iter().filter(|row| row.selected).count();
    let stop_refs = rows
        .iter()
        .filter(|row| row.selected)
        .map(|row| row.selected_stop_count)
        .sum::<usize>();
    println!("  selected T1 lines: {selected}/{route_budget}");
    println!("  selected stop refs: {stop_refs}/{stop_budget}");
    println!("  top city budget: {city_budget}");
    println!("  wrote selector: {}", output.display());
    println!(
        "  {:<8} {:>6} {:<9} {:>5} {:>5} {:>4} {:>5} {:<18} Reason",
        "Route", "Score", "Tier", "Stops", "Top25", "SLA", "Budget", "Decision"
    );
    for row in rows.iter().take(route_budget + 6) {
        println!(
            "  {:<8} {:>6.1} {:<9} {:>5} {:>5} {:>4} {:>5} {:<18} {}",
            row.route,
            row.score,
            row.tier,
            row.selected_stop_count,
            row.top_city_stop_count,
            row.sla_pair_count,
            row.budget_cost,
            row.decision,
            row.reason
        );
    }
    if gate {
        let failures = t1_line_selector_gate_failures(&rows, route_budget, stop_budget);
        if failures.is_empty() {
            println!("T1 line selector gate: PASS");
        } else {
            println!("T1 line selector gate: FAIL");
            for failure in failures.iter().take(10) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 line selector gate failed");
        }
    }

    Ok(())
}
