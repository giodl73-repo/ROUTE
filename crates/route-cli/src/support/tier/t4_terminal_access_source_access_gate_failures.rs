//! Helper `t4_terminal_access_source_access_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_source_access_gate_failures(
    rows: &[T4TerminalAccessSourceAccessRow],
    review_rows: &[T4TerminalAccessProofReviewRow],
) -> Vec<String> {
    let expected = review_rows
        .iter()
        .filter(|row| row.review_decision == "held-no-source-artifact")
        .map(|row| row.proof_review_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("terminal access source access has no held proof review rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access source access has {} rows but expected {} proof review rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.source_access_id.trim().is_empty()
            || row.proof_review_id.trim().is_empty()
            || row.proof_artifact_id.trim().is_empty()
            || row.acquisition_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.source_owner.trim().is_empty()
            || row.access_mode.trim().is_empty()
            || row.live_fetch_status.trim().is_empty()
            || row.required_source_metadata.trim().is_empty()
            || row.cache_policy_artifact.trim().is_empty()
            || row.source_access_blocker.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.proof_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete source access fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.proof_review_id.clone()) {
            failures.push(format!("{} appears more than once", row.proof_review_id));
        }
        if !expected.contains(row.proof_review_id.as_str()) {
            failures.push(format!(
                "{} is not a held proof review row",
                row.proof_review_id
            ));
        }
        if row.access_mode != "manual-or-cached-source-needed"
            || row.live_fetch_status != "unsupported-no-safe-terminal-access-fetcher"
            || row.evidence_artifact != "source-needed"
            || row.proof_acceptance_status != "not-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!("{} source access enabled proof", row.route));
        }
        if row.blocker_claims_before != "map;publication;upgrade"
            || row.blocker_claims_after != "map;publication;upgrade"
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} did not preserve blockers", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from source access"));
        }
    }
    failures
}

