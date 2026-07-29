//! Helper `t1_snapshot_plan_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_snapshot_plan_gate_failures(rows: &[T1SnapshotPlanRow]) -> Vec<&T1SnapshotPlanRow> {
    rows.iter()
        .filter(|row| !t1_snapshot_plan_row_has_contract(row))
        .collect()
}

