//! Helper `t2_beck_long_connector_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_long_connector_review_gate_failures(
    rows: &[T2BeckLongConnectorReviewRow],
    claim_rows: &[OptimizerClaimReviewRow],
    diagnostics: &[route_map::BeckT2DiagnosticRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T2"
            && row.blocker_family == "beck_long_connector"
            && row.total_claim_blockers > 0
    }) else {
        failures.push("missing T2 Beck long-connector optimizer claim-review row".to_string());
        return failures;
    };
    let expected_routes = claim_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect::<std::collections::BTreeSet<_>>();
    let eligible_routes = diagnostics
        .iter()
        .filter(|row| row.review_flag == "long-connector-review")
        .map(|row| route_display_key(row.corridor))
        .filter(|route| expected_routes.contains(route))
        .collect::<std::collections::BTreeSet<_>>();
    if eligible_routes.len() != expected_routes.len() {
        failures.push(format!(
            "eligible T2 long-connector routes = {}, expected {}",
            eligible_routes.len(),
            expected_routes.len()
        ));
    }
    if rows.len() != expected_routes.len() {
        failures.push(format!(
            "T2 long-connector review has {} rows but expected {}",
            rows.len(),
            expected_routes.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.connector_review_id.trim().is_empty()
            || row.claim_review_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.trunk.trim().is_empty()
            || row.start_trunk.trim().is_empty()
            || row.end_trunk.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.service_label.trim().is_empty()
            || row.split_anchor.trim().is_empty()
            || row.review_flag.trim().is_empty()
            || row.connector_basis.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete long-connector review fields",
                row.route
            ));
        }
        if !seen.insert(row.route.clone()) {
            failures.push(format!("{} appears more than once", row.route));
        }
        if !expected_routes.contains(&row.route) {
            failures.push(format!(
                "{} is not in the T2 long-connector claim row",
                row.route
            ));
        }
        if row.claim_review_id != claim_row.claim_review_id
            || row.review_flag != "long-connector-review"
            || row.review_decision != "long-connector-policy-required"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid long-connector state", row.route));
        }
        if row.blocker_claims_before != claim_row.blocked_claims
            || row.blocker_claims_after != claim_row.blocked_claims
            || row.blocker_count_before != 1
            || row.blocker_count_after != 1
            || row.claim_blocker_delta != 0
        {
            failures.push(format!(
                "{} reduced long-connector claim blockers",
                row.route
            ));
        }
        if row.next_artifact != "data/t2-beck-long-connector-policy.csv" {
            failures.push(format!("{} points at wrong next artifact", row.route));
        }
    }
    for expected_route in expected_routes {
        if !seen.contains(&expected_route) {
            failures.push(format!(
                "{expected_route} missing from T2 long-connector review"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != claim_row.total_claim_blockers {
        failures.push(format!(
            "T2 long-connector review preserves {total_after} blockers but claim row has {}",
            claim_row.total_claim_blockers
        ));
    }
    failures
}
