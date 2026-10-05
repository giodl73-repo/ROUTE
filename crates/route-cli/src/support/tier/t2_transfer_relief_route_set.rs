//! Helper `t2_transfer_relief_route_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_transfer_relief_route_set(
    rows: &[T2BeckTransferComplexityBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}
