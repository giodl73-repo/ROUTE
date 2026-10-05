//! Helper `load_t1_line_selector`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_line_selector(path: &Path) -> Result<Vec<T1LineSelectorInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
