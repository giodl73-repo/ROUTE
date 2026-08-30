//! Helper `print_t4_terminal_access_proof_source_capture_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_proof_source_capture_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofSourceCaptureRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.capture_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof source-capture rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}
