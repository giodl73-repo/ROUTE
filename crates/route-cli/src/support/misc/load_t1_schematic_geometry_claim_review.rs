//! Helper `load_t1_schematic_geometry_claim_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_schematic_geometry_claim_review(
    path: &Path,
) -> Result<Vec<T1SchematicGeometryClaimReviewRow>> {
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
