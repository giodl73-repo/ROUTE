//! Helper `t2_stitched_member_proof_review_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_review_docket_gate_failures(
    rows: &[T2StitchedMemberProofReviewDocketRow],
    attachment_rows: &[T2StitchedMemberProofArtifactAttachmentRow],
) -> Vec<String> {
    let expected = attachment_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| row.artifact_attachment_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures
            .push("stitched member proof review has no source-needed attachment rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member proof review has {} rows but expected {} attachment rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.proof_review_id.trim().is_empty()
            || row.artifact_attachment_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.source_artifact_reference.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.proof_acceptance_status.trim().is_empty()
            || row.candidate_disposition_status.trim().is_empty()
            || row.optimization_return_status.trim().is_empty()
            || row.review_reason.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete proof-review fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.artifact_attachment_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.artifact_attachment_id
            ));
        }
        if !expected.contains(row.artifact_attachment_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed artifact-attachment row",
                row.artifact_attachment_id
            ));
        }
        if row.source_artifact_reference != "source-needed"
            || row.review_decision != "held-no-source-artifact"
            || row.proof_acceptance_status != "not-accepted"
            || row.candidate_disposition_status != "not-ready-for-disposition"
            || row.optimization_return_status != "return-to-optimizer-held-known"
            || row.next_artifact != "data/tier-optimizer-runs.csv"
            || row.validation_status != "review"
        {
            failures.push(format!("{} proof review promoted evidence", row.route));
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
            failures.push(format!("{expected_id} missing from proof review"));
        }
    }
    failures
}
