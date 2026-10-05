//! Helper `t2_game_publication_evidence_policy_acceptance_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_policy_acceptance_gate_failures(
    rows: &[T2GamePublicationEvidencePolicyAcceptanceRow],
    policy_rows: &[T2GamePublicationEvidencePolicyRow],
) -> Vec<String> {
    let expected = policy_rows
        .iter()
        .filter(|row| {
            row.evidence_policy_decision == "publication-evidence-policy-authored-review"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| row.scenario_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_blockers = policy_rows
        .iter()
        .filter(|row| expected.contains(&row.scenario_id))
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures
            .push("T2 game publication evidence policy acceptance has no policy rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game publication evidence policy acceptance has {} rows but expected {}",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.acceptance_id.trim().is_empty()
            || row.policy_id.trim().is_empty()
            || row.scenario_id.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.t2_map_id.trim().is_empty()
            || row.accepted_required_evidence.trim().is_empty()
            || row.accepted_policy_treatment.trim().is_empty()
            || row.acceptance_decision.trim().is_empty()
            || row.publication_treatment.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete policy acceptance fields",
                row.scenario_id
            ));
        }
        if !seen.insert(row.scenario_id.clone()) {
            failures.push(format!("{} appears more than once", row.scenario_id));
        }
        if !expected.contains(&row.scenario_id) {
            failures.push(format!(
                "{} is not in the T2 game publication evidence policy rows",
                row.scenario_id
            ));
        }
        if row.acceptance_decision != "publication-evidence-policy-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid acceptance state", row.scenario_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!(
                "{} reduced game publication policy acceptance blockers",
                row.scenario_id
            ));
        }
        if row.next_artifact != "data/t2-game-publication-evidence-blocker-relief.csv" {
            failures.push(format!("{} points at wrong next artifact", row.scenario_id));
        }
    }
    for expected_scenario in expected {
        if !seen.contains(&expected_scenario) {
            failures.push(format!(
                "{expected_scenario} missing from T2 game publication evidence policy acceptance"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != expected_blockers {
        failures.push(format!(
            "T2 game publication evidence policy acceptance preserves {total_after} blockers but policy rows have {expected_blockers}"
        ));
    }
    failures
}
