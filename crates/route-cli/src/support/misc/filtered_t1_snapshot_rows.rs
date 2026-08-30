//! Helper `filtered_t1_snapshot_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn filtered_t1_snapshot_rows<'a>(
    rows: &'a [T1SnapshotPlanRow],
    priority: Option<&str>,
) -> Vec<&'a T1SnapshotPlanRow> {
    rows.iter()
        .filter(|row| {
            priority
                .map(|priority| row.priority_band.eq_ignore_ascii_case(priority))
                .unwrap_or(true)
        })
        .collect()
}
