//! Helper `load_t3_t4_access_gaps`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t3_t4_access_gaps(path: &Path) -> Result<Vec<T3T4AccessGapRow>> {
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
