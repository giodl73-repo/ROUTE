//! Helper `load_t4_terminal_access_proof_acquisition`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t4_terminal_access_proof_acquisition(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofAcquisitionRow>> {
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
