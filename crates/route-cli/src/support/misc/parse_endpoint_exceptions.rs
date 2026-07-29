//! Helper `parse_endpoint_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_endpoint_exceptions<R: std::io::Read>(reader: R) -> Result<Vec<EndpointExceptionRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

