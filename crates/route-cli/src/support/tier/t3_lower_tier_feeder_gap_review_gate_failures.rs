//! Helper `t3_lower_tier_feeder_gap_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_review_gate_failures(
    rows: &[T3LowerTierFeederGapReviewRow],
    backlog_rows: &[OptimizerResidualBlockerBacklogRow],
    access_gap_rows: &[T3T4AccessGapRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let Some(backlog_row) = backlog_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T3"
            && row.blocker_family == "lower_tier_feeder_gap"
            && row.total_claim_blockers > 0
    }) else {
        failures.push("missing T3 lower-tier feeder-gap residual backlog row".to_string());
        return failures;
    };
    let expected_routes = backlog_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect::<std::collections::BTreeSet<_>>();
    let eligible_routes = access_gap_rows
        .iter()
        .filter(|row| row.gap_class == "below-threshold-feeder" && row.promise_horizon_hours == 6)
        .map(|row| route_display_key(&row.route))
        .filter(|route| expected_routes.contains(route))
        .collect::<std::collections::BTreeSet<_>>();
    if eligible_routes.len() != expected_routes.len() {
        failures.push(format!(
            "eligible T3 feeder routes = {}, expected {}",
            eligible_routes.len(),
            expected_routes.len()
        ));
    }
    if rows.len() != expected_routes.len() {
        failures.push(format!(
            "T3 lower-tier feeder review has {} rows but expected {}",
            rows.len(),
            expected_routes.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        let route_key = route_display_key(&row.route);
        if row.feeder_review_id.trim().is_empty()
            || row.backlog_id.trim().is_empty()
            || row.gap_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.gap_class.trim().is_empty()
            || row.gap_reason.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete feeder review fields", row.route));
        }
        if !seen.insert(route_key.clone()) {
            failures.push(format!("{} appears more than once", row.route));
        }
        if !expected_routes.contains(&route_key) {
            failures.push(format!(
                "{} is not in the T3 lower-tier feeder backlog row",
                row.route
            ));
        }
        if row.backlog_id != backlog_row.backlog_id
            || row.gap_class != "below-threshold-feeder"
            || row.promise_horizon_hours != 6
            || row.review_decision != "lower-tier-feeder-policy-required"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid feeder review state", row.route));
        }
        if row.blocker_claims_before != backlog_row.blocked_claims
            || row.blocker_claims_after != backlog_row.blocked_claims
            || row.blocker_count_before != 1
            || row.blocker_count_after != 1
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced feeder blockers", row.route));
        }
    }
    for expected_route in expected_routes {
        if !seen.contains(&expected_route) {
            failures.push(format!(
                "{expected_route} missing from T3 lower-tier feeder review"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != backlog_row.total_claim_blockers {
        failures.push(format!(
            "T3 lower-tier feeder review preserves {total_after} blockers but backlog row has {}",
            backlog_row.total_claim_blockers
        ));
    }
    failures
}
