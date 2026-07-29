//! Helper `print_t4_terminal_access_proof_artifact_acquisition_target_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_proof_artifact_acquisition_target_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofArtifactAcquisitionTargetRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.acquisition_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof artifact acquisition-target rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

