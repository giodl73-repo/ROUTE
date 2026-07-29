//! Helper `t2_stitched_member_source_access_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_source_access_policy_gate_failures(
    rows: &[T2StitchedMemberSourceAccessPolicyRow],
    acquisition_rows: &[T2StitchedMemberEvidenceAcquisitionRow],
) -> Vec<String> {
    let expected = acquisition_rows
        .iter()
        .filter(|row| row.acquisition_status == "source-needed")
        .map(|row| row.acquisition_docket_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push(
            "stitched member source access policy has no source-needed acquisitions".to_string(),
        );
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member source access policy has {} rows but expected {} acquisition rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.access_policy_id.trim().is_empty()
            || row.acquisition_docket_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.source_owner.trim().is_empty()
            || row.access_mode.trim().is_empty()
            || row.live_fetch_status.trim().is_empty()
            || row.required_source_metadata.trim().is_empty()
            || row.cache_policy_artifact.trim().is_empty()
            || row.source_access_blocker.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete source access policy fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.acquisition_docket_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.acquisition_docket_id
            ));
        }
        if !expected.contains(row.acquisition_docket_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed acquisition row",
                row.acquisition_docket_id
            ));
        }
        if row.access_mode != "manual-or-cached-source-needed"
            || row.live_fetch_status != "unsupported-no-safe-stitched-member-fetcher"
            || row.evidence_artifact != "source-needed"
            || row.acquisition_status != "source-needed"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} source access policy enabled evidence",
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
            failures.push(format!("{expected_id} missing from source access policy"));
        }
    }
    failures
}

