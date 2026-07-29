//! Helper `load_game_t2_service_overlays`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_game_t2_service_overlays(path: &Path) -> Result<Vec<GameT2ServiceOverlayRow>> {
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

