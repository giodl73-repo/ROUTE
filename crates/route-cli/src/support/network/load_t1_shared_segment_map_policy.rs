//! Helper `load_t1_shared_segment_map_policy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_shared_segment_map_policy(
    path: &Path,
) -> Result<Vec<T1SharedSegmentMapPolicyRow>> {
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
