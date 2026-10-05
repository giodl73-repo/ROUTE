//! Helper `print_t2_bundle_repair_queue_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_repair_queue_summary(output: &Path, rows: &[T2BundleRepairQueueRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.bundle_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} bundle repair queue rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}
