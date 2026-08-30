//! Helper `tier_pavement_funding_evidence_review_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_review_docket_gate_failures(
    rows: &[TierPavementFundingEvidenceReviewDocketRow],
    attachment_rows: &[TierPavementFundingEvidenceArtifactAttachmentRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = attachment_rows
        .iter()
        .filter(|row| {
            row.attachment_status == "source-needed"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| row.artifact_attachment_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if expected.is_empty() {
        failures.push("funding evidence review has no held artifact-attachment rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "funding evidence review has {} rows but expected {} artifact-attachment rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.funding_evidence_review_id.trim().is_empty()
            || row.artifact_attachment_id.trim().is_empty()
            || row.evidence_contract_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.attached_artifact.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.evidence_review_status.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.review_reason.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete funding evidence review row",
                row.state, row.route
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
                "{} is not a held artifact-attachment row",
                row.artifact_attachment_id
            ));
        }
        if row.attached_artifact != "none" || row.review_decision != "held-no-attached-artifact" {
            failures.push(format!(
                "{} {} reviews an unattached funding artifact incorrectly",
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
                "{} {} is relief eligible before accepted evidence",
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
            failures.push(format!(
                "missing funding evidence review row for {expected_id}"
            ));
        }
    }
    failures
}
