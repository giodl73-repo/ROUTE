//! Helper `load_t1_sla_pairs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_sla_pairs(path: &Path) -> Result<Vec<T1SlaPairRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
