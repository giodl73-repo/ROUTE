//! Helper `standards_inventory_row_has_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn standards_inventory_row_has_contract(row: &StandardsInventoryRow) -> bool {
    let status = row.source_status.trim().to_ascii_lowercase();
    let status_is_labeled = matches!(
        status.as_str(),
        "implemented" | "partial" | "source_needed" | "access_gated" | "planned"
    );
    !row.standard_id.trim().is_empty()
        && !row.inventory_name.trim().is_empty()
        && !row.source_kind.trim().is_empty()
        && status_is_labeled
        && !row.current_artifact.trim().is_empty()
        && !row.coverage_scope.trim().is_empty()
        && !row.blocking_gap.trim().is_empty()
        && !row.next_step.trim().is_empty()
}
