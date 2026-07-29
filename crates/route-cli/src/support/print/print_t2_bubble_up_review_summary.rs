//! Helper `print_t2_bubble_up_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bubble_up_review_summary(output: &Path, rows: &[T2BubbleUpReviewRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bubble-up review rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

