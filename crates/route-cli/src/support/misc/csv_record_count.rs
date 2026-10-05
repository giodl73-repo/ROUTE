//! Helper `csv_record_count`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn csv_record_count(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let mut reader = csv::Reader::from_path(path)?;
    Ok(reader.records().count())
}
