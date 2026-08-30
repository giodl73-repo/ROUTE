//! Helper `t2_game_publication_relief_scenario_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_relief_scenario_set(
    rows: &[T2GamePublicationEvidenceBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| row.scenario_id.clone())
        .collect()
}
