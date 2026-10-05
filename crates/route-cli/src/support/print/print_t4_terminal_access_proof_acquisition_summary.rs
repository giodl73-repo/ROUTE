//! Helper `print_t4_terminal_access_proof_acquisition_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_proof_acquisition_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofAcquisitionRow],
) {
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
}
