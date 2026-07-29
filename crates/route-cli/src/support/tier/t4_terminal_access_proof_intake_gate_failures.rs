//! Helper `t4_terminal_access_proof_intake_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_intake_gate_failures(
    rows: &[T4TerminalAccessProofIntakeRow],
    access_rows: &[T4TerminalAccessSourceAccessRow],
) -> Vec<String> {
    let expected = access_rows
        .iter()
        .filter(|row| row.evidence_artifact == "source-needed")
        .map(|row| row.source_access_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("terminal access proof intake has no source-needed access rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access proof intake has {} rows but expected {} source-access rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.proof_intake_id.trim().is_empty()
            || row.source_access_id.trim().is_empty()
            || row.proof_review_id.trim().is_empty()
            || row.proof_artifact_id.trim().is_empty()
            || row.acquisition_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.required_artifact_fields.trim().is_empty()
            || row.required_contact_statement.trim().is_empty()
            || row.proof_artifact.trim().is_empty()
            || row.proof_status.trim().is_empty()
            || row.proof_blocker.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete proof intake fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.source_access_id.clone()) {
            failures.push(format!("{} appears more than once", row.source_access_id));
        }
        if !expected.contains(row.source_access_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed source-access row",
                row.source_access_id
            ));
        }
        if row.proof_artifact != "source-needed"
            || row.proof_status != "source-needed"
            || row.validation_status != "review"
        {
            failures.push(format!("{} proof intake accepted evidence", row.route));
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
            failures.push(format!("{expected_id} missing from proof intake"));
        }
    }
    failures
}

