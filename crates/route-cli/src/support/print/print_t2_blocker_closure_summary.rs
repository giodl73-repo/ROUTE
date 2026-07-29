//! Helper `print_t2_blocker_closure_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_blocker_closure_summary(output: &Path, rows: &[T2BlockerClosureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.blocker_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 blocker closure rows to {}",
        rows.len(),
        output.display()
    );
    for (blocker_class, count) in counts {
        println!("  {blocker_class}: {count}");
    }
}

