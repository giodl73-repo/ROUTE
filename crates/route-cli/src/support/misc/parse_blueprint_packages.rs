//! Helper `parse_blueprint_packages`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_blueprint_packages<R: std::io::Read>(
    reader: R,
) -> Result<Vec<BlueprintPackageRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}
