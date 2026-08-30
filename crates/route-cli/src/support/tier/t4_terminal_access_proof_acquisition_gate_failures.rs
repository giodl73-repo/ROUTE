//! Helper `t4_terminal_access_proof_acquisition_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_acquisition_gate_failures(
    rows: &[T4TerminalAccessProofAcquisitionRow],
    review_rows: &[T4TerminalAccessEvidenceReviewRow],
) -> Vec<String> {
    let expected = review_rows
        .iter()
        .filter(|row| row.review_decision == "held-source-needed")
        .map(|row| row.review_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("terminal access evidence review has no held-source-needed rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access proof acquisition has {} rows but expected {} held review rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.acquisition_id.trim().is_empty()
            || row.review_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.terminal_district_seed.trim().is_empty()
            || row.required_proof.trim().is_empty()
            || row.prohibited_seed_source.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.proof_artifact_status.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete acquisition fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.review_id.clone()) {
            failures.push(format!("{} appears more than once", row.review_id));
        }
        if !expected.contains(row.review_id.as_str()) {
            failures.push(format!("{} is not a held review row", row.review_id));
        }
        if row.acquisition_status != "source-needed"
            || row.proof_artifact_status != "not-attached"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} accepted proof during acquisition",
                row.queue_id
            ));
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
            failures.push(format!("{expected_id} missing from proof acquisition"));
        }
    }
    failures
}
