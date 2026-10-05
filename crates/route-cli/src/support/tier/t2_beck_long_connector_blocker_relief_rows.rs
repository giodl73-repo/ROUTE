//! Helper `t2_beck_long_connector_blocker_relief_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_long_connector_blocker_relief_rows(
    acceptance_rows: &[T2BeckLongConnectorPolicyAcceptanceRow],
) -> Vec<T2BeckLongConnectorBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "long-connector-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLongConnectorBlockerReliefRow {
            relief_id: format!("T2LONGRELIEF-{}", stable_id_fragment(&row.route)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            connector_band: row.connector_band.clone(),
            accepted_render_treatment: row.accepted_render_treatment.clone(),
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
