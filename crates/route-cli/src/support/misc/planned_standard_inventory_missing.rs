//! Helper `planned_standard_inventory_missing`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn planned_standard_inventory_missing<'a>(
    standards: &'a [StandardsProofRow],
    inventories: &[StandardsInventoryRow],
) -> Vec<&'a StandardsProofRow> {
    let covered = inventories
        .iter()
        .map(|row| row.standard_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    standards
        .iter()
        .filter(|row| row.evidence_level.eq_ignore_ascii_case("Planned"))
        .filter(|row| !covered.contains(row.standard_id.as_str()))
        .collect()
}

