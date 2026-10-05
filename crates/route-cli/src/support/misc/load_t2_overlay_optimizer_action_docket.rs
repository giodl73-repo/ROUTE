//! Helper `load_t2_overlay_optimizer_action_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_overlay_optimizer_action_docket(
    path: &Path,
) -> Result<Vec<T2OverlayOptimizerActionDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}
