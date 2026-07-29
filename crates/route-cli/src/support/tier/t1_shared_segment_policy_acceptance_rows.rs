//! Helper `t1_shared_segment_policy_acceptance_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_shared_segment_policy_acceptance_rows(
    policy_rows: &[T1SharedSegmentMapPolicyRow],
) -> Vec<T1SharedSegmentPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.map_policy_decision == "shared-segment-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
        })
        .map(|row| T1SharedSegmentPolicyAcceptanceRow {
            acceptance_id: format!("T1SHAREDACCEPT-{}", stable_id_fragment(&row.policy_id)),
            policy_id: row.policy_id.clone(),
            route_pair: row.route_pair.clone(),
            affected_routes: row.affected_routes.clone(),
            map_policy_decision: row.map_policy_decision.clone(),
            accepted_render_treatment: row.render_treatment.clone(),
            acceptance_status: "accepted-policy-ready-for-relief-replay".to_string(),
            acceptance_basis:
                "policy uses allowed interlined trunk or selected-transfer split treatment"
                    .to_string(),
            publication_status_before: row.publication_status.clone(),
            publication_status_after: "held-pending-blocker-relief-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t1-schematic-geometry-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route_pair.cmp(&right.route_pair));
    rows
}

