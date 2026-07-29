//! Helper `print_optimizer_residual_blocker_backlog_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_optimizer_residual_blocker_backlog_summary(
    output: &Path,
    rows: &[OptimizerResidualBlockerBacklogRow],
    details: bool,
) {
    let hard = rows
        .iter()
        .map(|row| row.total_hard_blockers)
        .sum::<usize>();
    let claim = rows
        .iter()
        .map(|row| row.total_claim_blockers)
        .sum::<usize>();
    let debt = rows
        .iter()
        .map(|row| row.total_budget_debt_count)
        .sum::<usize>();
    println!(
        "  wrote {} optimizer residual backlog rows to {}",
        rows.len(),
        output.display()
    );
    println!("  hard blockers: {hard}");
    println!("  claim blockers: {claim}");
    println!("  budget debt rows: {debt}");
    if details {
        println!();
        println!(
            "{:<18} {:<4} {:>5} {:>5} {:>5} {}",
            "Priority", "Tier", "Hard", "Claim", "Debt", "Family"
        );
        println!("{}", "-".repeat(100));
        for row in rows {
            println!(
                "{:<18} {:<4} {:>5} {:>5} {:>5} {}",
                row.priority_class,
                row.tier,
                row.total_hard_blockers,
                row.total_claim_blockers,
                row.total_budget_debt_count,
                row.blocker_family
            );
        }
    }
}

