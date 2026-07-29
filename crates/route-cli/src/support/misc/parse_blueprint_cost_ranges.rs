//! Helper `parse_blueprint_cost_ranges`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_blueprint_cost_ranges<R: std::io::Read>(reader: R) -> Result<Vec<BlueprintCostRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

