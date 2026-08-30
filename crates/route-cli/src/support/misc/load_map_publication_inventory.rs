//! Helper `load_map_publication_inventory`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_map_publication_inventory(
    path: &Path,
) -> Result<Vec<MapPublicationInventoryRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
