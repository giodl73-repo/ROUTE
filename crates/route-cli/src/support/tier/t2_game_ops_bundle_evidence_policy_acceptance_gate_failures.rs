//! Helper `t2_game_ops_bundle_evidence_policy_acceptance_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_policy_acceptance_gate_failures(
    rows: &[T2GameOpsBundleEvidencePolicyAcceptanceRow],
    policy_rows: &[T2GameOpsBundleEvidencePolicyRow],
) -> Vec<String> {
    let expected = policy_rows
        .iter()
        .filter(|row| {
            row.evidence_policy_decision == "bundle-evidence-policy-authored-review"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| row.policy_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = policy_rows
        .iter()
        .filter(|row| expected.contains(&row.policy_id))
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures
            .push("T2 game/ops bundle evidence policy acceptance has no policy rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game/ops bundle evidence policy acceptance has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let policy_by_id = policy_rows
        .iter()
        .map(|policy| (policy.policy_id.as_str(), policy))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.acceptance_id.trim().is_empty()
            || row.policy_id.trim().is_empty()
            || row.review_id.trim().is_empty()
            || row.decision_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.accepted_required_evidence.trim().is_empty()
            || row.accepted_policy_treatment.trim().is_empty()
            || row.acceptance_decision.trim().is_empty()
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
        if !expected.contains(&row.policy_id) {
            failures.push(format!("{} is not in the policy rows", row.policy_id));
        }
        if row.acceptance_decision != "bundle-evidence-policy-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid acceptance state", row.policy_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced acceptance blockers", row.policy_id));
        }
        if row.next_artifact != "data/t2-game-ops-bundle-evidence-blocker-relief.csv" {
            failures.push(format!("{} points at wrong next artifact", row.policy_id));
        }
        if let Some(policy) = policy_by_id.get(row.policy_id.as_str()) {
            let policy_has_qualification = !policy.qualification_gate_policy.trim().is_empty()
                || !policy.qualification_game_use.trim().is_empty()
                || !policy.qualification_effects.trim().is_empty();
            if policy_has_qualification
                && row.qualification_gate_policy.trim().is_empty()
                && row.qualification_game_use.trim().is_empty()
                && row.qualification_effects.trim().is_empty()
            {
                failures.push(format!(
                    "{} acceptance missing qualification semantics",
                    row.policy_id
                ));
            }
        }
    }
    for expected_id in expected {
        if !seen.contains(&expected_id) {
            failures.push(format!(
                "{expected_id} missing from T2 game/ops bundle evidence policy acceptance"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != expected_blockers {
        failures.push(format!(
            "T2 game/ops bundle evidence policy acceptance preserves {total_after} blockers but policy rows have {expected_blockers}"
        ));
    }
    failures
}

