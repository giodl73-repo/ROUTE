//! Helper `standards_inventory_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn standards_inventory_gate_failures(
    rows: &[StandardsInventoryRow],
) -> Vec<&StandardsInventoryRow> {
    rows.iter()
        .filter(|row| !standards_inventory_row_has_contract(row))
        .collect()
}
