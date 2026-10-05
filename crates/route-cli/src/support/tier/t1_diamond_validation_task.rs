//! Helper `t1_diamond_validation_task`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_diamond_validation_task(
    row: &T1DiamondValidationRow,
    category: &'static str,
    action: &str,
    source_action: Option<String>,
) -> T1DiamondValidationTask {
    T1DiamondValidationTask {
        priority_band: row.priority_band.clone(),
        category,
        site_id: row.site_id.clone(),
        intersection: row.intersection.clone(),
        location: row.location.clone(),
        action: action.to_string(),
        source_action,
    }
}
