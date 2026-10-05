//! Helper `t2_game_ops_bundle_evidence_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_policy_gate_failures(
    rows: &[T2GameOpsBundleEvidencePolicyRow],
    review_rows: &[T2GameOpsBundleEvidenceReviewRow],
) -> Vec<String> {
    let expected = review_rows
        .iter()
        .filter(|row| row.claim_blocker_delta == 0 && row.blocker_count_after > 0)
        .map(|row| row.review_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = review_rows
        .iter()
        .filter(|row| expected.contains(&row.review_id))
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("T2 game/ops bundle evidence policy has no review rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game/ops bundle evidence policy has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.policy_id.trim().is_empty()
            || row.review_id.trim().is_empty()
            || row.decision_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.repair_class.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.evidence_policy_decision.trim().is_empty()
            || row.policy_treatment.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete policy fields", row.review_id));
        }
        if !seen.insert(row.review_id.clone()) {
            failures.push(format!("{} appears more than once", row.review_id));
        }
        if !expected.contains(&row.review_id) {
            failures.push(format!("{} is not in the review rows", row.review_id));
        }
        if row.evidence_policy_decision != "bundle-evidence-policy-authored-review"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid policy state", row.review_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced policy blockers", row.review_id));
        }
        if row.blocker_count_after == 0 {
            failures.push(format!("{} lacks blocker count", row.review_id));
        }
        if row.next_artifact != "data/t2-game-ops-bundle-evidence-policy-acceptance.csv" {
            failures.push(format!("{} points at wrong next artifact", row.review_id));
        }
        if row.repair_class == "stop-chain"
            && row.service_repair_class == "not-service-class"
            && (row.qualification_gate_policy.trim().is_empty()
                || row.qualification_game_use.trim().is_empty())
        {
            failures.push(format!(
                "{} bundle-bound policy missing qualification semantics",
                row.review_id
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.qualification_gate_policy.trim().is_empty()
            && row.qualification_game_use.trim().is_empty()
        {
            failures.push(format!(
                "{} policy drops qualification contract",
                row.review_id
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(&expected_id) {
            failures.push(format!(
                "{expected_id} missing from T2 game/ops bundle evidence policy"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != expected_blockers {
        failures.push(format!(
            "T2 game/ops bundle evidence policy preserves {total_after} blockers but review rows have {expected_blockers}"
        ));
    }
    failures
}
