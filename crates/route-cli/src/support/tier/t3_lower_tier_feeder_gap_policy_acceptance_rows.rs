//! Helper `t3_lower_tier_feeder_gap_policy_acceptance_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_policy_acceptance_rows(
    policy_rows: &[T3LowerTierFeederGapPolicyRow],
) -> Vec<T3LowerTierFeederGapPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.feeder_policy_decision == "lower-tier-feeder-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T3LowerTierFeederGapPolicyAcceptanceRow {
            acceptance_id: format!("T3FEEDERACCEPT-{}", stable_id_fragment(&row.route)),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            score_band: row.score_band.clone(),
            accepted_map_treatment: row.map_treatment.clone(),
            accepted_evidence_treatment: row.evidence_treatment.clone(),
            accepted_upgrade_treatment: row.upgrade_treatment.clone(),
            acceptance_decision: "lower-tier-feeder-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t3-lower-tier-feeder-gap-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

