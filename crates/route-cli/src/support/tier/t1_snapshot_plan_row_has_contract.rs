//! Helper `t1_snapshot_plan_row_has_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_snapshot_plan_row_has_contract(row: &T1SnapshotPlanRow) -> bool {
    !row.site_id.trim().is_empty()
        && !row.intersection.trim().is_empty()
        && !row.priority_band.trim().is_empty()
        && !row.source_name.trim().is_empty()
        && row.source_health.trim() == "live/implemented/snapshot_only"
        && matches!(
            row.cadence.trim(),
            "daily" | "twice_daily" | "hourly" | "weekly"
        )
        && row.fetch_command.trim().starts_with("route t1-fetch-")
        && row.import_command.trim().starts_with("route t1-import-")
        && row
            .accumulate_command
            .trim()
            .starts_with("route t1-accumulate-events")
        && row.raw_output.trim().ends_with(".json")
        && row.normalized_output.trim().ends_with(".csv")
        && row.accumulated_output.trim().ends_with(".csv")
        && !row.blocking_gap.trim().is_empty()
        && !row.next_step.trim().is_empty()
}
