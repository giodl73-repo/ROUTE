//! Helper `load_t2_bundle_overlay_repair_delta`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t2_bundle_overlay_repair_delta(path: &Path) -> Result<Vec<T2BundleOverlayRepairDeltaRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

