//! Helper `largest_registry_district`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn largest_registry_district(
    registry_rows: &[T4TerminalContactProofSourceRegistryRow],
) -> Option<String> {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for row in registry_rows {
        *counts.entry(row.terminal_district.clone()).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0)))
        .map(|(district, _)| district)
}
