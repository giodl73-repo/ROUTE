//! Helper `load_standards_proof_ledger`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_standards_proof_ledger(path: &Path) -> Result<Vec<StandardsProofRow>> {
    let file = std::fs::File::open(path)?;
    parse_standards_proof_ledger(file)
}

