//! Helper `load_t2_beck_label_density_blocker_relief`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_beck_label_density_blocker_relief(
    path: &Path,
) -> Result<Vec<T2BeckLabelDensityBlockerReliefRow>> {
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

