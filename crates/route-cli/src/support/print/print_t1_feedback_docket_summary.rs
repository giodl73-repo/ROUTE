//! Helper `print_t1_feedback_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_feedback_docket_summary(output: &Path, rows: &[T1FeedbackDocketRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.t1_feedback_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T1 feedback rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}
