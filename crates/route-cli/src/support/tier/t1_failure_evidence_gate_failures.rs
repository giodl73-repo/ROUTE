//! Helper `t1_failure_evidence_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_failure_evidence_gate_failures(rows: &[T1FailureRow]) -> Vec<&T1FailureRow> {
    rows.iter()
        .filter(|row| !t1_failure_row_has_evidence_contract(row))
        .collect()
}

