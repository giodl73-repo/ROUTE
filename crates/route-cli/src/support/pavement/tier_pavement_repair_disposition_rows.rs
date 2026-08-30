//! Helper `tier_pavement_repair_disposition_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_repair_disposition_rows(
    repair_rows: &[TierPavementRepairDebtReviewRow],
) -> Vec<TierPavementRepairDispositionRow> {
    repair_rows
        .iter()
        .filter(|row| {
            row.source_priority == "A"
                && row.repair_debt_status == "confirmed-repair-debt"
                && row.validation_status == "review"
        })
        .map(|row| TierPavementRepairDispositionRow {
            disposition_id: format!(
                "PAVEMENTREPAIRDISPOSITION-{}",
                stable_id_fragment(&row.repair_review_id)
            ),
            repair_review_id: row.repair_review_id.clone(),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            blocked_member_count: row.blocked_member_count,
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            disposition: "repair-funding-required".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action:
                "create repair funding package or downgrade/exclude bundle before relief replay"
                    .to_string(),
            next_artifact: "data/tier-pavement-repair-disposition.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
