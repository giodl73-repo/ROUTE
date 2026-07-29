//! Helper `t2_beck_long_connector_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_long_connector_policy_rows(
    review_rows: &[T2BeckLongConnectorReviewRow],
) -> Vec<T2BeckLongConnectorPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "long-connector-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLongConnectorPolicyRow {
            policy_id: format!("T2LONGPOLICY-{}", stable_id_fragment(&row.route)),
            connector_review_id: row.connector_review_id.clone(),
            route: row.route.clone(),
            trunk_pair: format!("{}-{}", row.start_trunk, row.end_trunk),
            service_class: row.service_class.clone(),
            schematic_length_px: row.schematic_length_px,
            connector_band: t2_long_connector_band(row.schematic_length_px).to_string(),
            policy_basis: row.connector_basis.clone(),
            connector_policy_decision: "long-connector-policy-authored-review".to_string(),
            render_treatment:
                "preserve connector service but require trunk-interface labeling and explicit local-service beads"
                    .to_string(),
            promotion_treatment:
                "hold map promotion until accepted long-connector treatment is replayed"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

