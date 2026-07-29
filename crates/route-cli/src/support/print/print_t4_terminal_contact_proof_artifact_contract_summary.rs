//! Helper `print_t4_terminal_contact_proof_artifact_contract_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_contact_proof_artifact_contract_summary(
    output: &Path,
    rows: &[T4TerminalContactProofArtifactContractRow],
) {
    println!(
        "  wrote {} terminal contact proof artifact contract rows to {}",
        rows.len(),
        output.display()
    );
}

