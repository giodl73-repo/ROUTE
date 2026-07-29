//! Helper `parse_stop_candidates`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_stop_candidates<R: std::io::Read>(reader: R) -> Result<Vec<StopCandidateRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

