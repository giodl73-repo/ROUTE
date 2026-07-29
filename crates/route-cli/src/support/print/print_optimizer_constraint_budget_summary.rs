//! Helper `print_optimizer_constraint_budget_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_optimizer_constraint_budget_summary(
    output: &Path,
    rows: &[OptimizerConstraintBudgetRow],
    details: bool,
) {
    let total_debt_m = rows
        .iter()
        .map(|row| row.constraint_debt_cost_m)
        .sum::<f64>();
    let hard_blockers = rows.iter().map(|row| row.hard_blocker_count).sum::<usize>();
    let claim_blockers = rows
        .iter()
        .map(|row| row.claim_blocker_count)
        .sum::<usize>();
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.validation_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} optimizer constraint budget rows to {}",
        rows.len(),
        output.display()
    );
    println!("  constraint debt: ${total_debt_m:.2}M");
    println!("  hard blockers: {hard_blockers}");
    println!("  claim blockers: {claim_blockers}");
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:<18} {:>5} {:>5} {:>9} {}",
            "Tier", "Scope", "Status", "Hard", "Claim", "Cost $M", "Subject"
        );
        println!("{}", "-".repeat(110));
        for row in rows {
            println!(
                "{:<4} {:<8} {:<18} {:>5} {:>5} {:>9.2} {}",
                row.tier,
                row.subject_scope,
                row.validation_status,
                row.hard_blocker_count,
                row.claim_blocker_count,
                row.constraint_debt_cost_m,
                row.subject_id
            );
        }
    }
}

