//! Helper `parse_standards_inventory`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_standards_inventory<R: std::io::Read>(
    reader: R,
) -> Result<Vec<StandardsInventoryRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}
