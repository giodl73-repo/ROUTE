//! Helper `print_t2_overlay_p2_service_overlay_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_overlay_p2_service_overlay_review_summary(
    output: &Path,
    rows: &[T2OverlayP2ServiceOverlayReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts
            .entry(row.service_overlay_decision.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay P2 service-overlay review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}
