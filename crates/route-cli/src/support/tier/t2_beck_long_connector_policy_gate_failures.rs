//! Helper `t2_beck_long_connector_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_long_connector_policy_gate_failures(
    rows: &[T2BeckLongConnectorPolicyRow],
    review_rows: &[T2BeckLongConnectorReviewRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_routes = review_rows
        .iter()
        .filter(|row| row.review_decision == "long-connector-policy-required")
        .map(|row| row.route.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = review_rows
        .iter()
        .filter(|row| row.review_decision == "long-connector-policy-required")
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if expected_routes.is_empty() {
        failures.push("T2 long-connector policy has no review routes".to_string());
    }
    if rows.len() != expected_routes.len() {
        failures.push(format!(
            "T2 long-connector policy has {} rows but expected {} routes",
            rows.len(),
            expected_routes.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.policy_id.trim().is_empty()
            || row.connector_review_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.trunk_pair.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.connector_band.trim().is_empty()
            || row.policy_basis.trim().is_empty()
            || row.connector_policy_decision.trim().is_empty()
            || row.render_treatment.trim().is_empty()
            || row.promotion_treatment.trim().is_empty()
            || row.publication_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete long-connector policy fields",
                row.route
            ));
        }
        if !seen.insert(row.route.clone()) {
            failures.push(format!("{} appears more than once", row.route));
        }
        if !expected_routes.contains(&row.route) {
            failures.push(format!(
                "{} is not in the T2 long-connector review rows",
                row.route
            ));
        }
        if row.connector_policy_decision != "long-connector-policy-authored-review"
            || row.publication_status != "held-pending-policy-acceptance"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid connector policy state", row.route));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced connector policy blockers", row.route));
        }
        if row.next_artifact != "data/t2-beck-long-connector-policy-acceptance.csv" {
            failures.push(format!("{} points at wrong next artifact", row.route));
        }
    }
    for expected_route in expected_routes {
        if !seen.contains(&expected_route) {
            failures.push(format!(
                "{expected_route} missing from T2 long-connector policy"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != expected_blockers {
        failures.push(format!(
            "T2 long-connector policy preserves {total_after} blockers but review rows have {expected_blockers}"
        ));
    }
    failures
}
