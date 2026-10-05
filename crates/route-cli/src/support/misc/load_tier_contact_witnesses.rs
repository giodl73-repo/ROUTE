//! Helper `load_tier_contact_witnesses`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_contact_witnesses(path: &Path) -> Result<Vec<TierContactWitnessInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
