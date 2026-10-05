//! Helper `load_national_segment_registry`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_national_segment_registry(
    path: &Path,
) -> Result<Vec<NationalSegmentRegistryRow>> {
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
