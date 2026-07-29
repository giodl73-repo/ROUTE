//! Helper `print_t2_stitched_member_candidate_scope_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_stitched_member_candidate_scope_review_summary(
    output: &Path,
    rows: &[T2StitchedMemberCandidateScopeReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.scope_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member candidate scope review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

