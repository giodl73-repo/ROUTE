//! Helper `tier_pavement_funding_evidence_accepted_metadata_source_access_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_metadata_source_access_gate_failures(
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow],
    acquisition_rows: &[TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = acquisition_rows
        .iter()
        .filter(|row| {
            row.acquisition_status == "source-needed"
                && row.cache_status == "not-cached"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| row.accepted_metadata_artifact_acquisition_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if expected.is_empty() {
        failures.push(
            "funding evidence accepted metadata source access has no source-needed acquisitions"
                .to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "funding evidence accepted metadata source access has {} rows but expected {} acquisition rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.accepted_metadata_source_access_id.trim().is_empty()
            || row
                .accepted_metadata_artifact_acquisition_id
                .trim()
                .is_empty()
            || row.evidence_contract_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.source_owner.trim().is_empty()
            || row.access_mode.trim().is_empty()
            || row.cache_status.trim().is_empty()
            || row.live_fetch_status.trim().is_empty()
            || row.required_source_metadata.trim().is_empty()
            || row.cache_policy_artifact.trim().is_empty()
            || row.source_access_blocker.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete accepted metadata source-access row",
                row.state, row.route
            ));
        }
        if !seen.insert(row.accepted_metadata_artifact_acquisition_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.accepted_metadata_artifact_acquisition_id
            ));
        }
        if !expected.contains(row.accepted_metadata_artifact_acquisition_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed accepted metadata artifact-acquisition row",
                row.accepted_metadata_artifact_acquisition_id
            ));
        }
        if row.access_mode != "manual-or-cached-source-needed"
            || row.cache_status != "not-cached"
            || row.live_fetch_status != "unsupported-no-safe-funding-commitment-fetcher"
            || row.evidence_artifact != "source-needed"
        {
            failures.push(format!(
                "{} {} source access enabled unsupported evidence",
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
                "missing accepted metadata source-access row for {expected_id}"
            ));
        }
    }
    failures
}
