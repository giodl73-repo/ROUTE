//! Helper `t2_regionalizer_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_regionalizer_gate_failures(rows: &[T2RegionalizerRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 regionalizer rows emitted".to_string());
        return failures;
    }
    let selected = rows
        .iter()
        .filter(|row| row.treatment_status == "selected-treatment")
        .count();
    if selected == 0 {
        failures.push("no selected T2 regional treatments".to_string());
    }
    failures
}
