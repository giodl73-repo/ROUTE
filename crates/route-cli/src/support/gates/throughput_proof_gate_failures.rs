//! Helper `throughput_proof_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn throughput_proof_gate_failures(rows: &[ThroughputProofRow]) -> Vec<&ThroughputProofRow> {
    rows.iter()
        .filter(|row| !throughput_proof_has_bounded_contract(row))
        .collect()
}

