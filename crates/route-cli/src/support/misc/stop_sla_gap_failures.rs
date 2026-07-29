//! Helper `stop_sla_gap_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_sla_gap_failures(rows: &[StopSlaRow], max_gap: f64) -> Vec<&StopSlaRow> {
    let mut failures = rows
        .iter()
        .filter(|row| row.max_stop_gap_miles > max_gap)
        .collect::<Vec<_>>();
    failures.sort_by(|a, b| b.max_stop_gap_miles.total_cmp(&a.max_stop_gap_miles));
    failures
}

