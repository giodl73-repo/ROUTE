//! Helper `load_pavement_standards`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_pavement_standards(path: &Path) -> Result<Vec<PavementStandardRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

