//! Helper `t2_game_publication_evidence_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_policy_gate_failures(
    rows: &[T2GamePublicationEvidencePolicyRow],
    review_rows: &[T2GamePublicationEvidenceReviewRow],
) -> Vec<String> {
    let expected = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "publication-evidence-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| row.scenario_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = review_rows
        .iter()
        .filter(|row| expected.contains(&row.scenario_id))
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("T2 game publication evidence policy has no review rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game publication evidence policy has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.policy_id.trim().is_empty()
            || row.game_review_id.trim().is_empty()
            || row.scenario_id.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.t2_map_id.trim().is_empty()
            || row.evidence_policy_basis.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.evidence_policy_decision.trim().is_empty()
            || row.policy_treatment.trim().is_empty()
            || row.publication_treatment.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete policy fields", row.scenario_id));
        }
        if !seen.insert(row.scenario_id.clone()) {
            failures.push(format!("{} appears more than once", row.scenario_id));
        }
        if !expected.contains(&row.scenario_id) {
            failures.push(format!("{} is not in the review rows", row.scenario_id));
        }
        if row.evidence_policy_decision != "publication-evidence-policy-authored-review"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid policy state", row.scenario_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced policy blockers", row.scenario_id));
        }
        if row.next_artifact != "data/t2-game-publication-evidence-policy-acceptance.csv" {
            failures.push(format!("{} points at wrong next artifact", row.scenario_id));
        }
    }
    for expected_scenario in expected {
        if !seen.contains(&expected_scenario) {
            failures.push(format!(
                "{expected_scenario} missing from T2 game publication evidence policy"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != expected_blockers {
        failures.push(format!(
            "T2 game publication evidence policy preserves {total_after} blockers but review rows have {expected_blockers}"
        ));
    }
    failures
}
