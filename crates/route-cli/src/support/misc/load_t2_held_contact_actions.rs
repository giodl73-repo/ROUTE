//! Helper `load_t2_held_contact_actions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_held_contact_actions(path: &Path) -> Result<Vec<T2HeldContactActionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
