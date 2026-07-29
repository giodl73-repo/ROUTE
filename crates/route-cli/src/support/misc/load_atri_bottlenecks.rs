//! Helper `load_atri_bottlenecks`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_atri_bottlenecks(path: &Path) -> Result<Vec<AtriBottleneckRow>> {
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

