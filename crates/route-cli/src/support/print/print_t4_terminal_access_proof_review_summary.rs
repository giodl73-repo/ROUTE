//! Helper `print_t4_terminal_access_proof_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_proof_review_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

