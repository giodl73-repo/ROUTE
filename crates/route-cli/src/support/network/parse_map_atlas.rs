//! Helper `parse_map_atlas`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_map_atlas<R: std::io::Read>(reader: R) -> Result<Vec<MapAtlasRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

