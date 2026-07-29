//! Helper `load_t1_topology_repairs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_topology_repairs(path: &Path) -> Result<Vec<T1TopologyRepairRow>> {
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

