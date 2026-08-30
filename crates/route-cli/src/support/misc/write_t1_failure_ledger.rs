//! Helper `write_t1_failure_ledger`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_t1_failure_ledger(path: &Path, rows: &[T1FailureRow]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}
