//! Helper `t1_design_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_design_review_gate_failures(rows: &[T1DesignReviewRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.iter().all(|row| !row.selected) {
        failures.push("no selected T1 design rows".to_string());
    }
    for row in rows {
        if row.selected && row.selected_stop_count == 0 {
            failures.push(format!("{} selected without stop chain", row.route));
        }
        if !row.selected && row.promise_count > 0 {
            failures.push(format!(
                "{} carries {} promise pairs but is not selected",
                row.route, row.promise_count
            ));
        }
    }
    failures
}
