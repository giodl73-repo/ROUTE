//! Helper `print_bundle_architecture_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_bundle_architecture_summary(output: &Path, rows: &[BundleArchitectureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.validation_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} bundle architecture rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

