//! Helper `t1_topology_repair_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_topology_repair_gate_failures(rows: &[T1TopologyRepairRow]) -> Vec<String> {
    let mut failures = Vec::new();
    for row in rows {
        if row.next_action.trim().is_empty() {
            failures.push(format!("{} has no next topology repair action", row.route));
        }
        if row.next_artifact.trim().is_empty() {
            failures.push(format!(
                "{} has no next topology repair artifact",
                row.route
            ));
        }
    }
    failures
}
