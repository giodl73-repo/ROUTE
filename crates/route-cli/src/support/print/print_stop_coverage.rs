//! Helper `print_stop_coverage`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_stop_coverage(tier: &str, rows: &[StopCoverageRow], blockers: bool) {
    let visible = rows
        .iter()
        .filter(|row| !blockers || !row.failures.is_empty())
        .collect::<Vec<_>>();
    let passing = rows.iter().filter(|row| row.failures.is_empty()).count();
    println!("  tier: {tier}");
    println!("  routes: {}", rows.len());
    println!("  passing stop plans: {passing}");
    println!("  blockers: {}", rows.len().saturating_sub(passing));
    println!();
    println!(
        "{:<8} {:>5} {:>8} {:<22} Status",
        "Route", "Stops", "S1/S2", "Class mix"
    );
    println!("{}", "-".repeat(72));
    for row in visible {
        println!(
            "{:<8} {:>5} {:>8} {:<22} {}",
            row.route,
            row.stop_count,
            row.major_stop_count,
            truncate_for_table(&row.classes, 22),
            if row.failures.is_empty() {
                "pass".to_string()
            } else {
                truncate_for_table(&row.failures.join("; "), 28)
            }
        );
    }
}

