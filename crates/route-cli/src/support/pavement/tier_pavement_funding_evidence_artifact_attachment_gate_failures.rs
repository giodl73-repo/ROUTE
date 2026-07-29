//! Helper `tier_pavement_funding_evidence_artifact_attachment_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_artifact_attachment_gate_failures(
    rows: &[TierPavementFundingEvidenceArtifactAttachmentRow],
    capture_rows: &[TierPavementFundingEvidenceSourceCaptureRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = capture_rows
        .iter()
        .filter(|row| {
            row.source_capture_status == "source-needed"
                && row.captured_artifact == "none"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| row.source_capture_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if expected.is_empty() {
        failures.push(
            "funding evidence artifact attachment has no held source-capture rows".to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "funding evidence artifact attachment has {} rows but expected {} source-capture rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.artifact_attachment_id.trim().is_empty()
            || row.source_capture_id.trim().is_empty()
            || row.evidence_contract_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.attachment_status.trim().is_empty()
            || row.attached_artifact.trim().is_empty()
            || row.evidence_review_status.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.attachment_blocker.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete funding evidence artifact-attachment row",
                row.state, row.route
            ));
        }
        if !seen.insert(row.source_capture_id.clone()) {
            failures.push(format!("{} appears more than once", row.source_capture_id));
        }
        if !expected.contains(row.source_capture_id.as_str()) {
            failures.push(format!(
                "{} is not a held source-needed source-capture row",
                row.source_capture_id
            ));
        }
        if row.attachment_status != "source-needed" {
            failures.push(format!(
                "{} {} has unsupported attachment status",
                row.state, row.route
            ));
        }
        if row.attached_artifact != "none" {
            failures.push(format!(
                "{} {} attaches unreviewed funding artifact",
                row.state, row.route
            ));
        }
        if row.evidence_review_status != "not-reviewed"
            || row.accepted_evidence_status != "not-accepted"
        {
            failures.push(format!(
                "{} {} accepts funding evidence prematurely",
                row.state, row.route
            ));
        }
        if row.relief_eligibility != "not-eligible-for-relief" {
            failures.push(format!(
                "{} {} is relief eligible before evidence review",
                row.state, row.route
            ));
        }
        if row.blocked_claims_before != row.blocked_claims_after || row.claim_blocker_delta != 0 {
            failures.push(format!(
                "{} {} changes blockers before relief",
                row.state, row.route
            ));
        }
        if row.minimum_commitment_amount_m <= 0.0 {
            failures.push(format!(
                "{} {} lacks commitment amount",
                row.state, row.route
            ));
        }
        if row.validation_status != "held" {
            failures.push(format!("{} {} is not held", row.state, row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("missing attachment row for {expected_id}"));
        }
    }
    failures
}

