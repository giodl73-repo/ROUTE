//! Helper `write_t1_failure_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_t1_failure_events(path: &Path, rows: &[T1FailureEventRow]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}
