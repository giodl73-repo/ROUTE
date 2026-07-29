//! Helper `print_t4_terminal_columbus_proof_intake_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_columbus_proof_intake_summary(
    output: &Path,
    rows: &[T4TerminalColumbusProofIntakeRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.proof_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} Columbus South proof intake rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

