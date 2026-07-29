//! Helper `print_t3_lower_tier_feeder_gap_policy_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_lower_tier_feeder_gap_policy_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapPolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

