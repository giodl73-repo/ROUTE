//! Helper `print_t2_bundle_readiness_disposition_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_readiness_disposition_summary(
    output: &Path,
    rows: &[T2BundleReadinessDispositionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.disposition.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness disposition rows to {}",
        rows.len(),
        output.display()
    );
    for (disposition, count) in counts {
        println!("  {disposition}: {count}");
    }
}
