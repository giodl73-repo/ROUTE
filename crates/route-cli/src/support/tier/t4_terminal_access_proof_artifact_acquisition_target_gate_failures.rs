//! Helper `t4_terminal_access_proof_artifact_acquisition_target_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_artifact_acquisition_target_gate_failures(
    rows: &[T4TerminalAccessProofArtifactAcquisitionTargetRow],
    review_rows: &[T4TerminalAccessProofAttachmentReviewRow],
) -> Vec<String> {
    let expected = review_rows
        .iter()
        .filter(|row| {
            row.source_artifact_reference == "source-needed"
                && row.review_decision == "held-no-source-artifact"
                && row.proof_acceptance_status == "not-accepted"
                && row.validation_status == "review"
        })
        .map(|row| row.attachment_review_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push(
            "terminal access proof artifact acquisition targets have no held attachment-review rows"
                .to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access proof artifact acquisition targets have {} rows but expected {} attachment-review rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.acquisition_target_id.trim().is_empty()
            || row.attachment_review_id.trim().is_empty()
            || row.artifact_attachment_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.candidate_source_owner.trim().is_empty()
            || row.required_artifact_fields.trim().is_empty()
            || row.prohibited_seed_source.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.cache_status.trim().is_empty()
            || row.source_artifact_reference.trim().is_empty()
            || row.proof_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete acquisition target fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.attachment_review_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.attachment_review_id
            ));
        }
        if !expected.contains(row.attachment_review_id.as_str()) {
            failures.push(format!(
                "{} is not a held attachment-review row",
                row.attachment_review_id
            ));
        }
        if row.acquisition_status != "source-needed"
            || row.cache_status != "not-cached"
            || row.source_artifact_reference != "source-needed"
            || row.proof_acceptance_status != "not-accepted"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} acquisition target accepted evidence",
                row.route
            ));
        }
        if row.prohibited_seed_source != "data/intermodal_terminals.csv" {
            failures.push(format!(
                "{} permits seed-source proof laundering",
                row.route
            ));
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
            failures.push(format!("{expected_id} missing from acquisition targets"));
        }
    }
    failures
}

