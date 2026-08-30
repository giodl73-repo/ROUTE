//! Helper `load_fema_tiles`.
#[allow(unused_imports)]
use crate::*;

/// Load FEMA SFHA tile counts from data/cache/fema_sfha_tile_counts.csv.
/// Returns an empty Vec if the file is not present or cannot be parsed.
pub(crate) fn load_fema_tiles() -> Vec<FemaTile> {
    let path = std::path::Path::new("data/cache/fema_sfha_tile_counts.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(file) = std::fs::File::open(path) else {
        return Vec::new();
    };
    parse_fema_tiles(file)
}
