//! Helper `load_t2_stitched_member_registry_handoff`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_stitched_member_registry_handoff(
    path: &Path,
) -> Result<Vec<T2StitchedMemberRegistryHandoffRow>> {
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

