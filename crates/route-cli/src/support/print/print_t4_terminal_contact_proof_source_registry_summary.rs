//! Helper `print_t4_terminal_contact_proof_source_registry_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_contact_proof_source_registry_summary(
    output: &Path,
    rows: &[T4TerminalContactProofSourceRegistryRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.registry_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} terminal contact proof source registry rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}
