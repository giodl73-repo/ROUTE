//! Helper `t1_shared_segment_policy_acceptance_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_shared_segment_policy_acceptance_gate_failures(
    rows: &[T1SharedSegmentPolicyAcceptanceRow],
    policy_rows: &[T1SharedSegmentMapPolicyRow],
) -> Vec<String> {
    let expected = policy_rows
        .iter()
        .filter(|row| {
            row.map_policy_decision == "shared-segment-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
        })
        .map(|row| row.policy_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = policy_rows
        .iter()
        .filter(|row| expected.contains(row.policy_id.as_str()))
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("T1 shared segment policy acceptance has no held policy rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T1 shared segment policy acceptance has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.acceptance_id.trim().is_empty()
            || row.policy_id.trim().is_empty()
            || row.route_pair.trim().is_empty()
            || row.affected_routes.trim().is_empty()
            || row.map_policy_decision.trim().is_empty()
            || row.accepted_render_treatment.trim().is_empty()
            || row.acceptance_status.trim().is_empty()
            || row.acceptance_basis.trim().is_empty()
            || row.publication_status_before.trim().is_empty()
            || row.publication_status_after.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete acceptance fields",
                row.policy_id
            ));
        }
        if !seen.insert(row.policy_id.clone()) {
            failures.push(format!("{} appears more than once", row.policy_id));
        }
        if !expected.contains(row.policy_id.as_str()) {
            failures.push(format!("{} is not an expected policy row", row.policy_id));
        }
        if row.acceptance_status != "accepted-policy-ready-for-relief-replay"
            || row.publication_status_before != "held-pending-policy-acceptance"
            || row.publication_status_after != "held-pending-blocker-relief-replay"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid acceptance state", row.policy_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!(
                "{} reduced blockers during acceptance",
                row.policy_id
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from policy acceptance"));
        }
    }
    let actual_blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if actual_blockers != expected_blockers {
        failures.push(format!(
            "T1 shared segment policy acceptance preserves {actual_blockers} blockers but expected {expected_blockers}"
        ));
    }
    failures
}
