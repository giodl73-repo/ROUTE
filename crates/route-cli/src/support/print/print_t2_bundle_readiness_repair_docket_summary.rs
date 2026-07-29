//! Helper `print_t2_bundle_readiness_repair_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_readiness_repair_docket_summary(
    output: &Path,
    rows: &[T2BundleReadinessRepairDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.readiness_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness repair docket rows to {}",
        rows.len(),
        output.display()
    );
    for (readiness_class, count) in counts {
        println!("  {readiness_class}: {count}");
    }
}

