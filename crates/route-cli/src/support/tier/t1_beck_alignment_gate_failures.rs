//! Helper `t1_beck_alignment_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_beck_alignment_gate_failures(rows: &[T1BeckAlignmentRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 Beck alignment rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if !row.validation_status.eq_ignore_ascii_case("pass") {
            failures.push(format!(
                "{} alignment_status={} selector_stops={} beck_stops={}",
                row.route, row.alignment_status, row.selector_stop_count, row.beck_stop_count
            ));
        }
    }
    failures
}

