//! Helper `print_tier_pavement_acquisition_plan_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_acquisition_plan_summary(
    output: &Path,
    rows: &[TierPavementAcquisitionPlanRow],
    details: bool,
) {
    let mut by_priority = std::collections::BTreeMap::<&str, usize>::new();
    let blocked_total: usize = rows.iter().map(|row| row.blocked_member_count).sum();
    for row in rows {
        *by_priority.entry(row.source_priority.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement acquisition rows to {}",
        rows.len(),
        output.display()
    );
    println!("  assigned pavement debt member coverage: {blocked_total}");
    for (priority, count) in by_priority {
        println!("  priority {priority}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<5} {:<3} {:>6} {:>7} {:<28} {}",
            "State", "Pri", "Routes", "Blocked", "Affected routes", "Action"
        );
        println!("{}", "-".repeat(120));
        for row in rows {
            println!(
                "{:<5} {:<3} {:>6} {:>7} {:<28} {}",
                row.state,
                row.source_priority,
                row.route_count,
                row.blocked_member_count,
                truncate_for_table(&row.affected_routes, 28),
                truncate_for_table(&row.acquisition_action, 54)
            );
        }
    }
}

