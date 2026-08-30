//! Helper `load_t4_terminal_contact_proof_source_registry`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t4_terminal_contact_proof_source_registry(
    path: &Path,
) -> Result<Vec<T4TerminalContactProofSourceRegistryRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
