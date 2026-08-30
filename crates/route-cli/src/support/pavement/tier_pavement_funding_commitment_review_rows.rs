//! Helper `tier_pavement_funding_commitment_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_commitment_review_rows(
    package_rows: &[TierPavementRepairFundingPackageRow],
) -> Vec<TierPavementFundingCommitmentReviewRow> {
    package_rows
        .iter()
        .filter(|row| {
            row.funding_package_status == "package-required"
                && row.funding_commitment_status == "unfunded"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingCommitmentReviewRow {
            commitment_review_id: format!(
                "PAVEMENTFUNDINGCOMMITMENT-{}",
                stable_id_fragment(&row.funding_package_id)
            ),
            funding_package_id: row.funding_package_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            funding_commitment_status: "no-accepted-commitment-attached".to_string(),
            accepted_commitment_artifact: "none".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "choose downgrade/exclusion or attach accepted funding commitment before relief replay".to_string(),
            next_artifact: "data/tier-pavement-funding-commitment-review.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
