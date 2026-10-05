//! Helper `t3_t4_pressure_intake_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_t4_pressure_intake_gate_failures(rows: &[T3T4PressureIntakeRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3/T4 pressure intake rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.intake_class.trim().is_empty()
            || row.intake_action.trim().is_empty()
            || row.target_tier.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete pressure intake", row.route));
        }
    }
    failures
}
