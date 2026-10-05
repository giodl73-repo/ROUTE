//! Helper `print_optimizer_claim_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_optimizer_claim_review_summary(
    output: &Path,
    rows: &[OptimizerClaimReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.total_claim_blockers)
        .sum::<usize>();
    println!(
        "  wrote {} optimizer claim review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
