//! Helper `t1_schematic_relief_route_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_schematic_relief_route_set(
    rows: &[T1SchematicGeometryBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .flat_map(|row| row.affected_routes.split(';'))
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect()
}
