//! Helper `print_t1_beck_alignment_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_beck_alignment_summary(output: &Path, rows: &[T1BeckAlignmentRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.alignment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} alignment rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}
