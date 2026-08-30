//! Helper `t4_terminal_access_proof_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_review_gate_failures(
    rows: &[T4TerminalAccessProofReviewRow],
    artifact_rows: &[T4TerminalAccessProofArtifactRow],
) -> Vec<String> {
    let expected = artifact_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| row.proof_artifact_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures
            .push("terminal access proof review has no source-needed artifact rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access proof review has {} rows but expected {} artifact rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.proof_review_id.trim().is_empty()
            || row.proof_artifact_id.trim().is_empty()
            || row.acquisition_id.trim().is_empty()
            || row.review_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.source_artifact_reference.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.proof_acceptance_status.trim().is_empty()
            || row.optimization_return_status.trim().is_empty()
            || row.review_reason.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete proof review fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.proof_artifact_id.clone()) {
            failures.push(format!("{} appears more than once", row.proof_artifact_id));
        }
        if !expected.contains(row.proof_artifact_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed proof artifact row",
                row.proof_artifact_id
            ));
        }
        if row.source_artifact_reference != "source-needed"
            || row.review_decision != "held-no-source-artifact"
            || row.proof_acceptance_status != "not-accepted"
            || row.optimization_return_status != "return-to-optimizer-held-known"
            || row.next_artifact != "data/tier-optimizer-runs.csv"
            || row.validation_status != "review"
        {
            failures.push(format!("{} proof review promoted evidence", row.queue_id));
        }
        if row.blocker_claims_before != "map;publication;upgrade"
            || row.blocker_claims_after != "map;publication;upgrade"
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} did not preserve blockers", row.queue_id));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from proof review"));
        }
    }
    failures
}
