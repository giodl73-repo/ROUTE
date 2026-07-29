//! Helper `t3_zone_map_ids`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_map_ids(atlas_rows: &[MapAtlasRow]) -> std::collections::BTreeSet<String> {
    atlas_rows
        .iter()
        .filter(|row| row.map_type == "t3-zone")
        .map(|row| row.map_id.clone())
        .collect()
}

