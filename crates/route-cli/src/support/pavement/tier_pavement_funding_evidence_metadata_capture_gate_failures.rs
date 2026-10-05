//! Helper `tier_pavement_funding_evidence_metadata_capture_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_metadata_capture_gate_failures(
    rows: &[TierPavementFundingEvidenceMetadataCaptureRow],
    intake_rows: &[TierPavementFundingEvidenceIntakeRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = intake_rows
        .iter()
        .filter(|row| {
            row.intake_status == "artifact-required"
                && row.evidence_artifact == "source-needed"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| row.funding_evidence_intake_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if expected.is_empty() {
        failures.push("funding evidence metadata capture has no held intake rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "funding evidence metadata capture has {} rows but expected {} intake rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.metadata_capture_id.trim().is_empty()
            || row.funding_evidence_intake_id.trim().is_empty()
            || row.evidence_contract_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.required_artifact_type.trim().is_empty()
            || row.metadata_capture_status.trim().is_empty()
            || row.captured_artifact.trim().is_empty()
            || row.captured_source_title.trim().is_empty()
            || row.captured_source_url.trim().is_empty()
            || row.captured_commitment_amount_m.trim().is_empty()
            || row.evidence_review_status.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete funding evidence metadata-capture row",
                row.state, row.route
            ));
        }
        if !seen.insert(row.funding_evidence_intake_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.funding_evidence_intake_id
            ));
        }
        if !expected.contains(row.funding_evidence_intake_id.as_str()) {
            failures.push(format!(
                "{} is not a held funding evidence intake row",
                row.funding_evidence_intake_id
            ));
        }
        if row.required_artifact_type != "accepted-full-cost-programming-or-dot-commitment"
            || row.metadata_capture_status != "source-needed"
            || row.captured_artifact != "none"
            || row.captured_source_title != "source-needed"
            || row.captured_source_url != "source-needed"
            || row.captured_commitment_amount_m != "source-needed"
        {
            failures.push(format!(
                "{} {} captures unsupported funding metadata",
                row.state, row.route
            ));
        }
        if row.evidence_review_status != "not-reviewed"
            || row.accepted_evidence_status != "not-accepted"
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
                "missing funding evidence metadata-capture row for {expected_id}"
            ));
        }
    }
    failures
}
