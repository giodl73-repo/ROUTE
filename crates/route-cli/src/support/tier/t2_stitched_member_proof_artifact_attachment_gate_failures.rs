//! Helper `t2_stitched_member_proof_artifact_attachment_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_artifact_attachment_gate_failures(
    rows: &[T2StitchedMemberProofArtifactAttachmentRow],
    capture_rows: &[T2StitchedMemberProofSourceCaptureRow],
) -> Vec<String> {
    let expected = capture_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| row.source_capture_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push(
            "stitched member artifact attachment has no source-needed capture rows".to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member artifact attachment has {} rows but expected {} capture rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.artifact_attachment_id.trim().is_empty()
            || row.source_capture_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.source_artifact_reference.trim().is_empty()
            || row.attachment_status.trim().is_empty()
            || row.evidence_review_status.trim().is_empty()
            || row.proof_acceptance_status.trim().is_empty()
            || row.attachment_blocker.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete artifact-attachment fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.source_capture_id.clone()) {
            failures.push(format!("{} appears more than once", row.source_capture_id));
        }
        if !expected.contains(row.source_capture_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed source-capture row",
                row.source_capture_id
            ));
        }
        if row.source_artifact_reference != "source-needed"
            || row.attachment_status != "source-needed"
            || row.evidence_review_status != "not-reviewed"
            || row.proof_acceptance_status != "not-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} artifact attachment accepted evidence",
                row.route
            ));
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
            failures.push(format!("{expected_id} missing from artifact attachment"));
        }
    }
    failures
}

