//! Helper `tier_pavement_funding_evidence_accepted_intake_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_intake_gate_failures(
    rows: &[TierPavementFundingEvidenceAcceptedIntakeRow],
    access_rows: &[TierPavementFundingEvidenceAcceptedSourceAccessRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = access_rows
        .iter()
        .filter(|row| {
            row.access_mode == "manual-or-cached-source-needed"
                && row.cache_status == "not-cached"
                && row.evidence_artifact == "source-needed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| row.accepted_source_access_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if expected.is_empty() {
        failures
            .push("funding evidence accepted intake has no held source-access rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "funding evidence accepted intake has {} rows but expected {} source-access rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.accepted_intake_id.trim().is_empty()
            || row.accepted_source_access_id.trim().is_empty()
            || row.evidence_contract_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.required_artifact_type.trim().is_empty()
            || row.required_source_metadata.trim().is_empty()
            || row.intake_status.trim().is_empty()
            || row.cache_status.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.evidence_review_status.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.intake_blocker.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete accepted intake row",
                row.state, row.route
            ));
        }
        if !seen.insert(row.accepted_source_access_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.accepted_source_access_id
            ));
        }
        if !expected.contains(row.accepted_source_access_id.as_str()) {
            failures.push(format!(
                "{} is not a held accepted source-access row",
                row.accepted_source_access_id
            ));
        }
        if row.required_artifact_type != "accepted-full-cost-programming-or-dot-commitment"
            || row.intake_status != "artifact-required"
            || row.cache_status != "not-cached"
            || row.evidence_artifact != "source-needed"
        {
            failures.push(format!(
                "{} {} has unsupported accepted intake status",
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
            failures.push(format!("missing accepted intake row for {expected_id}"));
        }
    }
    failures
}

