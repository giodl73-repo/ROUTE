//! Helper `load_optimizer_residual_blocker_backlog`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_optimizer_residual_blocker_backlog(
    path: &Path,
) -> Result<Vec<OptimizerResidualBlockerBacklogRow>> {
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
