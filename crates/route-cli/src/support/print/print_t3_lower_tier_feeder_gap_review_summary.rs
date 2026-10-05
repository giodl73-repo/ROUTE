//! Helper `print_t3_lower_tier_feeder_gap_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_lower_tier_feeder_gap_review_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
