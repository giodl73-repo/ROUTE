//! Helper `t2_game_ops_bundle_evidence_blocker_relief_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_blocker_relief_gate_failures(
    rows: &[T2GameOpsBundleEvidenceBlockerReliefRow],
    acceptance_rows: &[T2GameOpsBundleEvidencePolicyAcceptanceRow],
) -> Vec<String> {
    let expected = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "bundle-evidence-policy-accepted"
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
        failures.push(
            "T2 game/ops bundle evidence blocker relief has no accepted policy rows".to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game/ops bundle evidence blocker relief has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let acceptance_by_id = acceptance_rows
        .iter()
        .map(|acceptance| (acceptance.acceptance_id.as_str(), acceptance))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.relief_id.trim().is_empty()
            || row.acceptance_id.trim().is_empty()
            || row.policy_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.accepted_required_evidence.trim().is_empty()
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
        if row.next_artifact != "data/optimizer-constraint-ledger.csv" {
            failures.push(format!(
                "{} points at wrong next artifact",
                row.acceptance_id
            ));
        }
        if let Some(acceptance) = acceptance_by_id.get(row.acceptance_id.as_str()) {
            let acceptance_has_qualification =
                !acceptance.qualification_gate_policy.trim().is_empty()
                    || !acceptance.qualification_game_use.trim().is_empty()
                    || !acceptance.qualification_effects.trim().is_empty();
            if acceptance_has_qualification
                && row.qualification_gate_policy.trim().is_empty()
                && row.qualification_game_use.trim().is_empty()
                && row.qualification_effects.trim().is_empty()
            {
                failures.push(format!(
                    "{} relief missing qualification semantics",
                    row.acceptance_id
                ));
            }
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
            "T2 game/ops bundle evidence relief before/after = {actual_before}/{actual_after}, expected {expected_before}/0"
        ));
    }
    failures
}
