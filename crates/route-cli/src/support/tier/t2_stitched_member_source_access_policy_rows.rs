//! Helper `t2_stitched_member_source_access_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_source_access_policy_rows(
    acquisition_rows: &[T2StitchedMemberEvidenceAcquisitionRow],
) -> Vec<T2StitchedMemberSourceAccessPolicyRow> {
    let mut rows = acquisition_rows
        .iter()
        .filter(|row| row.acquisition_status == "source-needed")
        .map(|acquisition| T2StitchedMemberSourceAccessPolicyRow {
            access_policy_id: format!(
                "T2STITCHEDACCESS-{}",
                stable_id_fragment(&acquisition.acquisition_docket_id)
            ),
            acquisition_docket_id: acquisition.acquisition_docket_id.clone(),
            route: acquisition.route.clone(),
            candidate_segment_bundle_id: acquisition.candidate_segment_bundle_id.clone(),
            state_scope: acquisition.state_scope.clone(),
            source_owner: acquisition.source_owner.clone(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-stitched-member-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; state scope; route geometry statement"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "no safe live stitched-member route geometry fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            acquisition_status: acquisition.acquisition_status.clone(),
            blocked_claims_before: acquisition.blocked_claims_after.clone(),
            blocked_claims_after: acquisition.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

