//! Helper `print_t2_bundle_overlay_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_overlay_summary(output: &Path, rows: &[T2BundleOverlayRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.binding_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle overlay rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}
