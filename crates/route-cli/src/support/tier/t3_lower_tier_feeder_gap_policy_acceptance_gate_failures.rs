//! Helper `t3_lower_tier_feeder_gap_policy_acceptance_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_policy_acceptance_gate_failures(
    rows: &[T3LowerTierFeederGapPolicyAcceptanceRow],
    policy_rows: &[T3LowerTierFeederGapPolicyRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_routes = policy_rows
        .iter()
        .filter(|row| {
            row.feeder_policy_decision == "lower-tier-feeder-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
        })
        .map(|row| row.route.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = policy_rows
        .iter()
        .filter(|row| {
            row.feeder_policy_decision == "lower-tier-feeder-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
        })
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if expected_routes.is_empty() {
        failures.push("T3 lower-tier feeder policy acceptance has no policy rows".to_string());
    }
    if rows.len() != expected_routes.len() {
        failures.push(format!(
            "T3 lower-tier feeder policy acceptance has {} rows but expected {} routes",
            rows.len(),
            expected_routes.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.acceptance_id.trim().is_empty()
            || row.policy_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.score_band.trim().is_empty()
            || row.accepted_map_treatment.trim().is_empty()
            || row.accepted_evidence_treatment.trim().is_empty()
            || row.accepted_upgrade_treatment.trim().is_empty()
            || row.acceptance_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete feeder acceptance fields",
                row.route
            ));
        }
        if !seen.insert(row.route.clone()) {
            failures.push(format!("{} appears more than once", row.route));
        }
        if !expected_routes.contains(&row.route) {
            failures.push(format!(
                "{} is not in the T3 lower-tier feeder policy rows",
                row.route
            ));
        }
        if row.acceptance_decision != "lower-tier-feeder-policy-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid feeder acceptance state", row.route));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced feeder acceptance blockers", row.route));
        }
        if row.next_artifact != "data/t3-lower-tier-feeder-gap-blocker-relief.csv" {
            failures.push(format!("{} points at wrong next artifact", row.route));
        }
    }
    for expected_route in expected_routes {
        if !seen.contains(&expected_route) {
            failures.push(format!(
                "{expected_route} missing from T3 lower-tier feeder policy acceptance"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != expected_blockers {
        failures.push(format!(
            "T3 lower-tier feeder policy acceptance preserves {total_after} blockers but policy rows have {expected_blockers}"
        ));
    }
    failures
}

