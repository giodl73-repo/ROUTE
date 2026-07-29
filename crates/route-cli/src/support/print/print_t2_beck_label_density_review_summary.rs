//! Helper `print_t2_beck_label_density_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_beck_label_density_review_summary(output: &Path, rows: &[T2BeckLabelDensityReviewRow]) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck label-density review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

