//! Helper `t1_shared_segment_map_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_shared_segment_map_policy_gate_failures(
    rows: &[T1SharedSegmentMapPolicyRow],
    schematic_rows: &[T1SchematicGeometryClaimReviewRow],
) -> Vec<String> {
    let expected_pairs = schematic_rows
        .iter()
        .filter(|row| row.policy_action == "resolve-shared-segment-map-policy")
        .map(|row| shared_segment_pair_id(&row.route, &row.overlap_corridors))
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = schematic_rows
        .iter()
        .filter(|row| row.policy_action == "resolve-shared-segment-map-policy")
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected_pairs.is_empty() {
        failures.push("T1 shared segment policy has no schematic review pairs".to_string());
    }
    if rows.len() != expected_pairs.len() {
        failures.push(format!(
            "T1 shared segment policy has {} rows but expected {} pairs",
            rows.len(),
            expected_pairs.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.policy_id.trim().is_empty()
            || row.route_pair.trim().is_empty()
            || row.primary_route.trim().is_empty()
            || row.overlap_route.trim().is_empty()
            || row.affected_routes.trim().is_empty()
            || row.source_review_ids.trim().is_empty()
            || row.policy_basis.trim().is_empty()
            || row.map_policy_decision.trim().is_empty()
            || row.render_treatment.trim().is_empty()
            || row.selector_treatment.trim().is_empty()
            || row.publication_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete policy fields", row.route_pair));
        }
        if !seen.insert(row.route_pair.clone()) {
            failures.push(format!("{} appears more than once", row.route_pair));
        }
        if !expected_pairs.contains(&row.route_pair) {
            failures.push(format!(
                "{} is not an expected shared segment pair",
                row.route_pair
            ));
        }
        if row.map_policy_decision != "shared-segment-policy-authored-review"
            || row.publication_status != "held-pending-policy-acceptance"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid policy state", row.route_pair));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!(
                "{} reduced shared-segment blockers",
                row.route_pair
            ));
        }
    }
    for expected_pair in expected_pairs {
        if !seen.contains(&expected_pair) {
            failures.push(format!(
                "{expected_pair} missing from shared segment policy"
            ));
        }
    }
    let actual_blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if actual_blockers != expected_blockers {
        failures.push(format!(
            "T1 shared segment policy preserves {actual_blockers} blockers but expected {expected_blockers}"
        ));
    }
    failures
}
