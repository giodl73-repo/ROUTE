//! Helper `tier_pavement_downgrade_exclusion_decision_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_downgrade_exclusion_decision_rows(
    commitment_rows: &[TierPavementFundingCommitmentReviewRow],
) -> Vec<TierPavementDowngradeExclusionDecisionRow> {
    commitment_rows
        .iter()
        .filter(|row| {
            row.funding_commitment_status == "no-accepted-commitment-attached"
                && row.accepted_commitment_artifact == "none"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementDowngradeExclusionDecisionRow {
            downgrade_exclusion_decision_id: format!(
                "PAVEMENTDOWNGRADEEXCLUSION-{}",
                stable_id_fragment(&row.commitment_review_id)
            ),
            commitment_review_id: row.commitment_review_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            downgrade_decision: "no-downgrade-selected".to_string(),
            exclusion_decision: "no-exclusion-selected".to_string(),
            service_status: "held-at-current-tier".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action:
                "attach accepted funding evidence before relief replay or open a separate downgrade/exclusion authorization"
                    .to_string(),
            next_artifact: "data/tier-pavement-downgrade-exclusion-decision.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
