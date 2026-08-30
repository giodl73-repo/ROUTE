//! Helper `load_t2_bundle_repair_queue`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_bundle_repair_queue(path: &Path) -> Result<Vec<T2BundleRepairQueueRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
