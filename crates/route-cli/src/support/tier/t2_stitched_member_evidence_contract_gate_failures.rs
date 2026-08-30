//! Helper `t2_stitched_member_evidence_contract_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_evidence_contract_gate_failures(
    rows: &[T2StitchedMemberEvidenceContractRow],
    selection_rows: &[T2StitchedMemberSelectionDocketRow],
) -> Vec<String> {
    let expected = selection_rows
        .iter()
        .filter(|row| row.selection_decision == "evidence-needed")
        .map(|row| row.selection_docket_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("stitched member evidence contract has no evidence-needed rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member evidence contract has {} rows but expected {} selection rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.evidence_contract_id.trim().is_empty()
            || row.selection_docket_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.required_continuity_proof.trim().is_empty()
            || row.required_scope_proof.trim().is_empty()
            || row.required_source_proof.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete evidence contract fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.selection_docket_id.clone()) {
            failures.push(format!(
                "{} appears more than once",
                row.selection_docket_id
            ));
        }
        if !expected.contains(row.selection_docket_id.as_str()) {
            failures.push(format!(
                "{} is not an evidence-needed selection row",
                row.selection_docket_id
            ));
        }
        if row.evidence_status != "source-needed" || row.validation_status != "review" {
            failures.push(format!(
                "{} evidence contract satisfied evidence",
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
            failures.push(format!("{expected_id} missing from evidence contract"));
        }
    }
    failures
}
