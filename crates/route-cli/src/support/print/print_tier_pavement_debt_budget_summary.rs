//! Helper `print_tier_pavement_debt_budget_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_debt_budget_summary(
    output: &Path,
    rows: &[TierPavementDebtBudgetRow],
    details: bool,
) {
    let total_cost_m = rows.iter().map(|row| row.total_debt_cost_m).sum::<f64>();
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_class.entry(row.debt_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement debt budget rows to {}",
        rows.len(),
        output.display()
    );
    println!("  planning pavement debt: ${total_cost_m:.2}M");
    for (debt_class, count) in by_class {
        println!("  {debt_class}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:<18} {:>7} {:>10} {}",
            "Tier", "Route", "Debt", "Members", "Cost $M", "Bundle"
        );
        println!("{}", "-".repeat(104));
        for row in rows {
            println!(
                "{:<4} {:<8} {:<18} {:>7} {:>10.2} {}",
                row.tier,
                row.route,
                row.debt_class,
                row.blocked_member_count,
                row.total_debt_cost_m,
                row.segment_bundle_id
            );
        }
    }
}
