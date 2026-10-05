//! Helper `load_t4_terminal_access_proof_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t4_terminal_access_proof_review(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofReviewRow>> {
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
