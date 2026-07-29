//! Helper `print_t2_overlay_p1_structural_readiness_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_overlay_p1_structural_readiness_review_summary(
    output: &Path,
    rows: &[T2OverlayP1StructuralReadinessReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.readiness_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay P1 structural-readiness review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

