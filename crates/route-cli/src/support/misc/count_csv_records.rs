//! Helper `count_csv_records`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn count_csv_records(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let mut reader = csv::Reader::from_path(path)?;
    Ok(reader.records().count())
}
