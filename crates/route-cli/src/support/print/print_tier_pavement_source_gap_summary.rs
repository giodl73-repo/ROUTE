//! Helper `print_tier_pavement_source_gap_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_source_gap_summary(
    output: &Path,
    rows: &[TierPavementSourceGapRow],
    details: bool,
) {
    let mut by_tier = std::collections::BTreeMap::<&str, usize>::new();
    let mut blocker_total = 0usize;
    for row in rows {
        *by_tier.entry(row.tier.as_str()).or_default() += 1;
        blocker_total += row.blocker_count;
    }
    println!(
        "  wrote {} pavement source-gap rows to {}",
        rows.len(),
        output.display()
    );
    println!("  pavement debt member segments: {blocker_total}");
    for (tier, count) in by_tier {
        println!("  {tier}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:>7} {:>7} {:<22} {}",
            "Tier", "Route", "Members", "Blocked", "States", "Action"
        );
        println!("{}", "-".repeat(112));
        for row in rows {
            println!(
                "{:<4} {:<8} {:>7} {:>7} {:<22} {}",
                row.tier,
                row.route,
                row.member_count,
                row.blocker_count,
                truncate_for_table(&row.affected_states, 22),
                truncate_for_table(&row.source_action, 52)
            );
        }
    }
}

