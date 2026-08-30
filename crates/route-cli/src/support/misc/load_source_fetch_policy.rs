//! Helper `load_source_fetch_policy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_source_fetch_policy(path: &Path) -> Result<Vec<SourceFetchPolicyRow>> {
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
