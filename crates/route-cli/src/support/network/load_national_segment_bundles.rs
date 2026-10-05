//! Helper `load_national_segment_bundles`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_national_segment_bundles(path: &Path) -> Result<Vec<NationalSegmentBundleRow>> {
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
