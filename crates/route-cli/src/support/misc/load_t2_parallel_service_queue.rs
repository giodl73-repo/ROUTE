//! Helper `load_t2_parallel_service_queue`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_parallel_service_queue(
    path: &Path,
) -> Result<Vec<T2ParallelServiceQueueRow>> {
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
