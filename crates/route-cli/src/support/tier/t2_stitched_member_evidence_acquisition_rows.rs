//! Helper `t2_stitched_member_evidence_acquisition_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_evidence_acquisition_rows(
    contract_rows: &[T2StitchedMemberEvidenceContractRow],
) -> Vec<T2StitchedMemberEvidenceAcquisitionRow> {
    let mut rows = contract_rows
        .iter()
        .filter(|row| row.evidence_status == "source-needed")
        .map(|contract| {
            let owner = if contract.state_scope.trim().is_empty() {
                "state DOT".to_string()
            } else {
                format!("{} DOT", contract.state_scope)
            };
            T2StitchedMemberEvidenceAcquisitionRow {
                acquisition_docket_id: format!(
                    "T2STITCHEDACQUIRE-{}",
                    stable_id_fragment(&contract.evidence_contract_id)
                ),
                evidence_contract_id: contract.evidence_contract_id.clone(),
                route: contract.route.clone(),
                candidate_segment_bundle_id: contract.candidate_segment_bundle_id.clone(),
                state_scope: contract.state_scope.clone(),
                source_owner: owner.clone(),
                source_target: format!(
                    "{} route log, GIS centerline, or official route description for {} {}",
                    owner, contract.route, contract.state_scope
                ),
                acquisition_action:
                    "manual-source-request-or-cache-official-route-geometry-before-decision"
                        .to_string(),
                acquisition_status: "source-needed".to_string(),
                blocked_claims_before: contract.blocked_claims_after.clone(),
                blocked_claims_after: contract.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            }
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

