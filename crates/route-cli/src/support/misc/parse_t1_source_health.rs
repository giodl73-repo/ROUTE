//! Helper `parse_t1_source_health`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_t1_source_health<R: std::io::Read>(reader: R) -> Result<Vec<T1SourceHealthRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

