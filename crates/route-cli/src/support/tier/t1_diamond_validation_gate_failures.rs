//! Helper `t1_diamond_validation_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_diamond_validation_gate_failures(
    rows: &[T1DiamondValidationRow],
) -> Vec<&T1DiamondValidationRow> {
    rows.iter()
        .filter(|row| !t1_diamond_validation_row_has_contract(row))
        .collect()
}
