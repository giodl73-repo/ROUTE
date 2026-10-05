//! Helper `t3_lower_tier_feeder_gap_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_policy_rows(
    review_rows: &[T3LowerTierFeederGapReviewRow],
) -> Vec<T3LowerTierFeederGapPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "lower-tier-feeder-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T3LowerTierFeederGapPolicyRow {
            policy_id: format!("T3FEEDERPOLICY-{}", stable_id_fragment(&row.route)),
            feeder_review_id: row.feeder_review_id.clone(),
            gap_id: row.gap_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            score_band: t3_feeder_score_band(row.current_score).to_string(),
            policy_basis: format!(
                "{};required_evidence={};repair_action={}",
                row.gap_reason, row.required_evidence, row.repair_action
            ),
            feeder_policy_decision: "lower-tier-feeder-policy-authored-review".to_string(),
            map_treatment:
                "keep route below T3 feeder promotion until accepted score or terminal evidence exists"
                    .to_string(),
            evidence_treatment:
                "require score-threshold proof or terminal-access evidence before any claim relief"
                    .to_string(),
            upgrade_treatment:
                "hold upgrade framing as T4 or evidence-needed unless policy acceptance authorizes T3 feeder treatment"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t3-lower-tier-feeder-gap-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
