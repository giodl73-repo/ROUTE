//! Helper `load_optimizer_constraint_budget`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_optimizer_constraint_budget(
    path: &Path,
) -> Result<Vec<OptimizerConstraintBudgetRow>> {
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
