//! Helper `load_map_publication_scope_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_map_publication_scope_decision(path: &Path) -> Result<Vec<MapPublicationScopeDecisionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

