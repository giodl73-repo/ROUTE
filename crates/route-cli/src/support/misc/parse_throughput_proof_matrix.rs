//! Helper `parse_throughput_proof_matrix`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_throughput_proof_matrix<R: std::io::Read>(reader: R) -> Result<Vec<ThroughputProofRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

