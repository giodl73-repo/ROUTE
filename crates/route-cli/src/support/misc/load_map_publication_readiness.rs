//! Helper `load_map_publication_readiness`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_map_publication_readiness(path: &Path) -> Result<Vec<MapPublicationReadinessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

