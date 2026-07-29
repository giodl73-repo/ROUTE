//! Helper `t2_stitched_member_evidence_contract_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_evidence_contract_rows(
    selection_rows: &[T2StitchedMemberSelectionDocketRow],
) -> Vec<T2StitchedMemberEvidenceContractRow> {
    let mut rows = selection_rows
        .iter()
        .filter(|row| row.selection_decision == "evidence-needed")
        .map(|selection| T2StitchedMemberEvidenceContractRow {
            evidence_contract_id: format!(
                "T2STITCHEDEVIDENCE-{}",
                stable_id_fragment(&selection.selection_docket_id)
            ),
            selection_docket_id: selection.selection_docket_id.clone(),
            route: selection.route.clone(),
            candidate_segment_bundle_id: selection.candidate_segment_bundle_id.clone(),
            state_scope: selection.state_scope.clone(),
            required_continuity_proof:
                "document continuous service relationship between candidate bundle and blocked stitched route"
                    .to_string(),
            required_scope_proof:
                "document why the state-scoped candidate belongs in or outside the blocked service"
                    .to_string(),
            required_source_proof:
                "cite authoritative route geometry or agency source before in-scope or rejected status"
                    .to_string(),
            evidence_status: "source-needed".to_string(),
            blocked_claims_before: selection.blocked_claims_after.clone(),
            blocked_claims_after: selection.blocked_claims_after.clone(),
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

