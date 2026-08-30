//! Helper `print_t3_lower_tier_feeder_gap_blocker_relief_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_lower_tier_feeder_gap_blocker_relief_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}
