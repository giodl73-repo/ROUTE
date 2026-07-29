//! Helper `parse_standards_proof_ledger`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_standards_proof_ledger<R: std::io::Read>(reader: R) -> Result<Vec<StandardsProofRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

