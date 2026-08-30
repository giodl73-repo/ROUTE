//! Helper `print_t2_regionalizer_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_regionalizer_summary(output: &Path, rows: &[T2RegionalizerRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.treatment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} regionalizer rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}
