//! Helper `print_t4_terminal_access_proof_attachment_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_proof_attachment_review_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofAttachmentReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof attachment-review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

