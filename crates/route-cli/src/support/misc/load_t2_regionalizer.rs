//! Helper `load_t2_regionalizer`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_regionalizer(path: &Path) -> Result<Vec<T2RegionalizerRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

