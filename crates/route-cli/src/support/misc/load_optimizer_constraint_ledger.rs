//! Helper `load_optimizer_constraint_ledger`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_optimizer_constraint_ledger(
    path: &Path,
) -> Result<Vec<OptimizerConstraintLedgerRow>> {
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
