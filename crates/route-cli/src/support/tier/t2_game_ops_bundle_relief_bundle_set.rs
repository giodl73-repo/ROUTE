//! Helper `t2_game_ops_bundle_relief_bundle_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_relief_bundle_set(
    rows: &[T2GameOpsBundleEvidenceBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| row.segment_bundle_id.clone())
        .collect()
}
