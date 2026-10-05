//! Helper `t3_lower_tier_feeder_gap_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_review_rows(
    backlog_rows: &[OptimizerResidualBlockerBacklogRow],
    access_gap_rows: &[T3T4AccessGapRow],
) -> Vec<T3LowerTierFeederGapReviewRow> {
    let Some(backlog_row) = backlog_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T3"
            && row.blocker_family == "lower_tier_feeder_gap"
            && row.total_claim_blockers > 0
    }) else {
        return Vec::new();
    };
    let expected_routes = backlog_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect::<std::collections::BTreeSet<_>>();
    let mut rows = access_gap_rows
        .iter()
        .filter(|row| {
            row.gap_class == "below-threshold-feeder"
                && row.promise_horizon_hours == 6
                && expected_routes.contains(&route_display_key(&row.route))
        })
        .map(|row| T3LowerTierFeederGapReviewRow {
            feeder_review_id: format!("T3FEEDERREVIEW-{}", stable_id_fragment(&row.route)),
            backlog_id: backlog_row.backlog_id.clone(),
            gap_id: row.gap_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            current_score: row.current_score,
            constraint_adjusted_score: row.constraint_adjusted_score,
            promise_horizon_hours: row.promise_horizon_hours,
            gap_class: row.gap_class.clone(),
            gap_reason: row.gap_reason.clone(),
            required_evidence: row.required_evidence.clone(),
            repair_action: row.repair_action.clone(),
            review_decision: "lower-tier-feeder-policy-required".to_string(),
            blocker_claims_before: backlog_row.blocked_claims.clone(),
            blocker_claims_after: backlog_row.blocked_claims.clone(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
