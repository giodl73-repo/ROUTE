//! Helper `t4_terminal_access_proof_artifact_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_artifact_gate_failures(
    rows: &[T4TerminalAccessProofArtifactRow],
    acquisition_rows: &[T4TerminalAccessProofAcquisitionRow],
) -> Vec<String> {
    let expected = acquisition_rows
        .iter()
        .filter(|row| row.proof_artifact_status == "not-attached")
        .map(|row| row.acquisition_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("terminal access proof acquisition has no not-attached rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access proof artifacts has {} rows but expected {} acquisition rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.proof_artifact_id.trim().is_empty()
            || row.acquisition_id.trim().is_empty()
            || row.review_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.terminal_district_seed.trim().is_empty()
            || row.required_proof.trim().is_empty()
            || row.source_artifact_reference.trim().is_empty()
            || row.attachment_status.trim().is_empty()
            || row.evidence_review_status.trim().is_empty()
            || row.proof_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete proof artifact fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.acquisition_id.clone()) {
            failures.push(format!("{} appears more than once", row.acquisition_id));
        }
        if !expected.contains(row.acquisition_id.as_str()) {
            failures.push(format!(
                "{} is not a not-attached acquisition row",
                row.acquisition_id
            ));
        }
        if row.source_artifact_reference != "source-needed"
            || row.attachment_status != "source-needed"
            || row.evidence_review_status != "not-reviewed"
            || row.proof_acceptance_status != "not-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!("{} accepted proof during attachment", row.queue_id));
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
            failures.push(format!("{expected_id} missing from proof artifacts"));
        }
    }
    failures
}

