//! Helper `t3_lower_tier_feeder_gap_blocker_relief_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_blocker_relief_rows(
    acceptance_rows: &[T3LowerTierFeederGapPolicyAcceptanceRow],
) -> Vec<T3LowerTierFeederGapBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "lower-tier-feeder-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T3LowerTierFeederGapBlockerReliefRow {
            relief_id: format!("T3FEEDERRELIEF-{}", stable_id_fragment(&row.route)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            score_band: row.score_band.clone(),
            accepted_map_treatment: row.accepted_map_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
