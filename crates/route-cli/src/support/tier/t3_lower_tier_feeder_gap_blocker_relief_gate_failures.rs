//! Helper `t3_lower_tier_feeder_gap_blocker_relief_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_lower_tier_feeder_gap_blocker_relief_gate_failures(
    rows: &[T3LowerTierFeederGapBlockerReliefRow],
    acceptance_rows: &[T3LowerTierFeederGapPolicyAcceptanceRow],
) -> Vec<String> {
    let expected = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "lower-tier-feeder-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| row.acceptance_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_before = acceptance_rows
        .iter()
        .filter(|row| expected.contains(row.acceptance_id.as_str()))
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures
            .push("T3 lower-tier feeder blocker relief has no accepted policy rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T3 lower-tier feeder blocker relief has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.relief_id.trim().is_empty()
            || row.acceptance_id.trim().is_empty()
            || row.policy_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.score_band.trim().is_empty()
            || row.accepted_map_treatment.trim().is_empty()
            || row.relief_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.ledger_replay_status.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete relief fields",
                row.acceptance_id
            ));
        }
        if !seen.insert(row.acceptance_id.clone()) {
            failures.push(format!("{} appears more than once", row.acceptance_id));
        }
        if !expected.contains(row.acceptance_id.as_str()) {
            failures.push(format!(
                "{} is not an expected acceptance row",
                row.acceptance_id
            ));
        }
        if row.relief_decision != "relief-ready-for-constraint-ledger-replay"
            || row.ledger_replay_status != "pending-optimizer-constraint-ledger-replay"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid relief state", row.acceptance_id));
        }
        if !row.blocker_claims_after.is_empty()
            || row.blocker_count_after != 0
            || row.claim_blocker_delta != -(row.blocker_count_before as isize)
        {
            failures.push(format!(
                "{} did not reduce blockers to zero",
                row.acceptance_id
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from blocker relief"));
        }
    }
    let actual_before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let actual_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if actual_before != expected_before || actual_after != 0 {
        failures.push(format!(
            "T3 lower-tier feeder relief before/after = {actual_before}/{actual_after}, expected {expected_before}/0"
        ));
    }
    failures
}
