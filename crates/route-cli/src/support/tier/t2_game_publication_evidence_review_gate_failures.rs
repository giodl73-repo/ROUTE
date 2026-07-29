//! Helper `t2_game_publication_evidence_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_review_gate_failures(
    rows: &[T2GamePublicationEvidenceReviewRow],
    claim_rows: &[OptimizerClaimReviewRow],
    hook_rows: &[T2ScenarioHookRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T2"
            && row.blocker_family == "game_ops_publication_readiness"
            && row.total_claim_blockers > 0
    }) else {
        failures.push("missing T2 game publication optimizer claim-review row".to_string());
        return failures;
    };
    let expected_scenarios = claim_row
        .representative_subjects
        .split(';')
        .filter(|scenario| !scenario.trim().is_empty())
        .map(str::to_string)
        .collect::<std::collections::BTreeSet<_>>();
    let eligible_scenarios = hook_rows
        .iter()
        .map(|row| row.scenario_id.clone())
        .filter(|scenario| expected_scenarios.contains(scenario))
        .collect::<std::collections::BTreeSet<_>>();
    if eligible_scenarios.len() != expected_scenarios.len() {
        failures.push(format!(
            "eligible T2 game publication scenarios = {}, expected {}",
            eligible_scenarios.len(),
            expected_scenarios.len()
        ));
    }
    if rows.len() != expected_scenarios.len() {
        failures.push(format!(
            "T2 game publication review has {} rows but expected {}",
            rows.len(),
            expected_scenarios.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.game_review_id.trim().is_empty()
            || row.claim_review_id.trim().is_empty()
            || row.scenario_id.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.t2_map_id.trim().is_empty()
            || row.player_decision.trim().is_empty()
            || row.evidence_hold.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete review fields", row.scenario_id));
        }
        if !seen.insert(row.scenario_id.clone()) {
            failures.push(format!("{} appears more than once", row.scenario_id));
        }
        if !expected_scenarios.contains(&row.scenario_id) {
            failures.push(format!(
                "{} is not in the T2 game publication claim row",
                row.scenario_id
            ));
        }
        if row.claim_review_id != claim_row.claim_review_id
            || row.review_decision != "publication-evidence-policy-required"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid review state", row.scenario_id));
        }
        if row.blocker_claims_before != claim_row.blocked_claims
            || row.blocker_claims_after != claim_row.blocked_claims
            || row.blocker_count_before != 1
            || row.blocker_count_after != 1
            || row.claim_blocker_delta != 0
        {
            failures.push(format!(
                "{} reduced game publication claim blockers",
                row.scenario_id
            ));
        }
        if row.next_artifact != "data/t2-game-publication-evidence-policy.csv" {
            failures.push(format!("{} points at wrong next artifact", row.scenario_id));
        }
    }
    for expected_scenario in expected_scenarios {
        if !seen.contains(&expected_scenario) {
            failures.push(format!(
                "{expected_scenario} missing from T2 game publication review"
            ));
        }
    }
    failures
}

