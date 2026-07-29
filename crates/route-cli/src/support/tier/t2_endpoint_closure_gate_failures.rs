//! Helper `t2_endpoint_closure_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_endpoint_closure_gate_failures(rows: &[T2EndpointClosureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_endpoint_closures__" {
        let row = &rows[0];
        if row.endpoint_action != "endpoint-closure-clear" || row.validation_status != "pass" {
            failures.push("endpoint closure clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.endpoint_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete endpoint closure", row.route));
        }
    }
    failures
}

