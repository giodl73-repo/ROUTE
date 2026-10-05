//! Helper `print_t3_t4_access_gap_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_t4_access_gap_summary(output: &Path, rows: &[T3T4AccessGapRow]) {
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_surface = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_class.entry(row.gap_class.as_str()).or_default() += 1;
        *by_surface.entry(row.source_surface.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3/T4 access gap rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in by_class {
        println!("  {class}: {count}");
    }
    for (surface, count) in by_surface {
        println!("  {surface}: {count}");
    }
}
