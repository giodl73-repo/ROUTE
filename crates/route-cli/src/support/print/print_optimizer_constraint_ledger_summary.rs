//! Helper `print_optimizer_constraint_ledger_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_optimizer_constraint_ledger_summary(
    output: &Path,
    rows: &[OptimizerConstraintLedgerRow],
    details: bool,
) {
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_behavior = std::collections::BTreeMap::<&str, usize>::new();
    let total_debt_m = rows.iter().map(|row| row.budget_cost_m).sum::<f64>();
    for row in rows {
        *by_class.entry(row.constraint_class.as_str()).or_default() += 1;
        *by_behavior.entry(row.behavior_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} optimizer constraint rows to {}",
        rows.len(),
        output.display()
    );
    println!("  constraint debt: ${total_debt_m:.2}M");
    for (class, count) in by_class {
        println!("  {class}: {count}");
    }
    for (behavior, count) in by_behavior {
        println!("  {behavior}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<28} {:<16} {:<8} {:>9} {}",
            "Tier", "Class", "Behavior", "Status", "Cost $M", "Subject"
        );
        println!("{}", "-".repeat(110));
        for row in rows {
            println!(
                "{:<4} {:<28} {:<16} {:<8} {:>9.2} {}",
                row.tier,
                row.constraint_class,
                row.behavior_type,
                row.validation_status,
                row.budget_cost_m,
                row.subject_id
            );
        }
    }
}

