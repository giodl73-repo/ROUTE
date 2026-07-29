//! Helper `t2_stitched_member_proof_source_capture_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_source_capture_gate_failures(
    rows: &[T2StitchedMemberProofSourceCaptureRow],
    intake_rows: &[T2StitchedMemberProofIntakeRow],
) -> Vec<String> {
    let expected = intake_rows
        .iter()
        .filter(|row| row.proof_artifact == "source-needed")
        .map(|row| row.proof_intake_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures
            .push("stitched member source capture has no source-needed intake rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member source capture has {} rows but expected {} intake rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.source_capture_id.trim().is_empty()
            || row.proof_intake_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.source_artifact_reference.trim().is_empty()
            || row.source_artifact_type.trim().is_empty()
            || row.capture_status.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.capture_blocker.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete source-capture fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.proof_intake_id.clone()) {
            failures.push(format!("{} appears more than once", row.proof_intake_id));
        }
        if !expected.contains(row.proof_intake_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed proof-intake row",
                row.proof_intake_id
            ));
        }
        if row.source_artifact_reference != "source-needed"
            || row.capture_status != "source-needed"
            || row.evidence_acceptance_status != "not-reviewed"
            || row.validation_status != "review"
        {
            failures.push(format!("{} source capture accepted evidence", row.route));
        }
        if row.blocked_claims_before != "game;incident;publication;upgrade"
            || row.blocked_claims_after != "game;incident;publication;upgrade"
            || row.blocker_delta != 0
        {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from source capture"));
        }
    }
    failures
}

