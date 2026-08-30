//! Helper `print_t2_game_ops_bundle_evidence_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_game_ops_bundle_evidence_review_summary(
    output: &Path,
    rows: &[T2GameOpsBundleEvidenceReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 game/ops bundle evidence review rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}
