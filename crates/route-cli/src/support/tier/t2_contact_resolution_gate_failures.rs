//! Helper `t2_contact_resolution_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_contact_resolution_gate_failures(rows: &[T2ContactResolutionRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 contact resolution rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.resolution_action.trim().is_empty()
            || row.resolution_basis.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete T2 contact resolution contract",
                row.route
            ));
        }
    }
    failures
}
