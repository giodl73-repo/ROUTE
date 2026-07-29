//! Helper `print_t2_beck_transfer_complexity_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_beck_transfer_complexity_review_summary(
    output: &Path,
    rows: &[T2BeckTransferComplexityReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck transfer-complexity review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

