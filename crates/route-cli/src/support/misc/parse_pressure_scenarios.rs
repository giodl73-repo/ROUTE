//! Helper `parse_pressure_scenarios`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_pressure_scenarios<R: std::io::Read>(reader: R) -> Result<Vec<PressureScenarioRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

