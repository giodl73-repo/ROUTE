//! Helper `load_tier_pavement_acquisition_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_pavement_acquisition_docket(
    path: &Path,
) -> Result<Vec<TierPavementAcquisitionDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
