//! Helper `t2_beck_long_connector_policy_acceptance_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_long_connector_policy_acceptance_rows(
    policy_rows: &[T2BeckLongConnectorPolicyRow],
) -> Vec<T2BeckLongConnectorPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.connector_policy_decision == "long-connector-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLongConnectorPolicyAcceptanceRow {
            acceptance_id: format!("T2LONGACCEPT-{}", stable_id_fragment(&row.route)),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            connector_band: row.connector_band.clone(),
            accepted_render_treatment: row.render_treatment.clone(),
            accepted_promotion_treatment: row.promotion_treatment.clone(),
            acceptance_decision: "long-connector-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
