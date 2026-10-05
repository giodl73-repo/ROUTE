//! Helper `stop_coverage_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_coverage_gate_failures(rows: &[StopCoverageRow]) -> Vec<String> {
    rows.iter()
        .filter(|row| !row.failures.is_empty())
        .map(|row| format!("{}: {}", row.route, row.failures.join("; ")))
        .collect()
}
