//! Helper `load_map_atlas`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_map_atlas(path: &Path) -> Result<Vec<MapAtlasRow>> {
    let file = std::fs::File::open(path)?;
    parse_map_atlas(file)
}
