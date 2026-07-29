//! Helper `print_t2_bundle_overlay_repair_target_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_overlay_repair_target_summary(
    output: &Path,
    rows: &[T2BundleOverlayRepairTargetRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle overlay repair target rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}

