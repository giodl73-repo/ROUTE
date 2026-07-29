//! Helper `t1_diamond_validation_row_has_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_diamond_validation_row_has_contract(row: &T1DiamondValidationRow) -> bool {
    let analyzer = row.analyzer_status.trim().to_ascii_lowercase();
    let geometry = row.manual_geometry_status.trim().to_ascii_lowercase();
    let alternate = row.alternate_capacity_status.trim().to_ascii_lowercase();
    let observed = row.observed_failure_status.trim().to_ascii_lowercase();
    let validation = row.validation_status.trim().to_ascii_lowercase();

    !row.site_id.trim().is_empty()
        && EXPECTED_T1_DIAMOND_SITES.contains(&row.site_id.as_str())
        && !row.intersection.trim().is_empty()
        && !row.location.trim().is_empty()
        && !row.priority_band.trim().is_empty()
        && row.anchor_lon.is_finite()
        && row.anchor_lat.is_finite()
        && matches!(analyzer.as_str(), "recognized" | "missing" | "conflict")
        && matches!(
            geometry.as_str(),
            "validated" | "heuristic" | "pending" | "conflict"
        )
        && matches!(alternate.as_str(), "validated" | "heuristic" | "pending")
        && matches!(observed.as_str(), "empirical" | "modeled" | "source_needed")
        && matches!(
            validation.as_str(),
            "validated" | "heuristic" | "pending" | "conflict"
        )
        && !row.current_artifact.trim().is_empty()
        && !row.next_validation_step.trim().is_empty()
        && (validation == "validated" || !row.blocking_gap.trim().is_empty())
}

