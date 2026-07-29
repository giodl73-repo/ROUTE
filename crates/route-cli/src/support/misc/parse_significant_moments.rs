//! Helper `parse_significant_moments`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_significant_moments<R: std::io::Read>(reader: R) -> Result<Vec<SignificantMomentRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

