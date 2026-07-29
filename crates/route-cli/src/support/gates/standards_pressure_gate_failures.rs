//! Helper `standards_pressure_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn standards_pressure_gate_failures(rows: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    rows.iter()
        .filter(|row| !standards_pressure_row_has_contract(row))
        .collect()
}

