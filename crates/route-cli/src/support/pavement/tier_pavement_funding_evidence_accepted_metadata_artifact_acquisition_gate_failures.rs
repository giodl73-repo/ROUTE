//! Helper `tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_gate_failures(
    rows: &[TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow],
    review_rows: &[TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "held-no-attached-artifact"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| row.accepted_metadata_attachment_review_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if expected.is_empty() {
        failures.push(
            "funding evidence accepted metadata artifact acquisition has no held review rows"
                .to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "funding evidence accepted metadata artifact acquisition has {} rows but expected {} review rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row
            .accepted_metadata_artifact_acquisition_id
            .trim()
            .is_empty()
            || row.accepted_metadata_attachment_review_id.trim().is_empty()
            || row.evidence_contract_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.required_artifact_type.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.cache_status.trim().is_empty()
            || row.candidate_source_owner.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.acquisition_reason.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete accepted metadata artifact-acquisition row",
                row.state, row.route
            ));
        }
        if !seen.insert(row.accepted_metadata_attachment_review_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.accepted_metadata_attachment_review_id
            ));
        }
        if !expected.contains(row.accepted_metadata_attachment_review_id.as_str()) {
            failures.push(format!(
                "{} is not a held accepted metadata attachment-review row",
                row.accepted_metadata_attachment_review_id
            ));
        }
        if row.required_artifact_type != "accepted-full-cost-programming-or-dot-commitment"
            || row.acquisition_status != "source-needed"
            || row.cache_status != "not-cached"
        {
            failures.push(format!(
                "{} {} has unsupported accepted metadata artifact-acquisition status",
                row.state, row.route
            ));
        }
        if row.accepted_evidence_status != "not-accepted"
            || row.relief_eligibility != "not-eligible-for-relief"
        {
            failures.push(format!(
                "{} {} accepts evidence or relief prematurely",
                row.state, row.route
            ));
        }
        if row.claim_blocker_delta != 0 {
            failures.push(format!(
                "{} {} changes blockers before relief",
                row.state, row.route
            ));
        }
        if row.validation_status != "held" {
            failures.push(format!("{} {} is not held", row.state, row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "missing accepted metadata artifact-acquisition row for {expected_id}"
            ));
        }
    }
    failures
}
