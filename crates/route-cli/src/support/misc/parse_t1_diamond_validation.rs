//! Helper `parse_t1_diamond_validation`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_t1_diamond_validation<R: std::io::Read>(
    reader: R,
) -> Result<Vec<T1DiamondValidationRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}
