//! Helper `t2_stitched_member_proof_intake_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_proof_intake_gate_failures(
    rows: &[T2StitchedMemberProofIntakeRow],
    access_rows: &[T2StitchedMemberSourceAccessPolicyRow],
) -> Vec<String> {
    let expected = access_rows
        .iter()
        .filter(|row| row.evidence_artifact == "source-needed")
        .map(|row| row.access_policy_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("stitched member proof intake has no source-needed access rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member proof intake has {} rows but expected {} access rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.proof_intake_id.trim().is_empty()
            || row.access_policy_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.candidate_segment_bundle_id.trim().is_empty()
            || row.state_scope.trim().is_empty()
            || row.required_artifact_fields.trim().is_empty()
            || row.required_geometry_statement.trim().is_empty()
            || row.proof_artifact.trim().is_empty()
            || row.proof_status.trim().is_empty()
            || row.proof_blocker.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete proof-intake fields",
                row.route, row.candidate_segment_bundle_id
            ));
        }
        if !seen.insert(row.access_policy_id.clone()) {
            failures.push(format!("{} appears more than once", row.access_policy_id));
        }
        if !expected.contains(row.access_policy_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed access-policy row",
                row.access_policy_id
            ));
        }
        if row.proof_artifact != "source-needed"
            || row.proof_status != "source-needed"
            || row.validation_status != "review"
        {
            failures.push(format!("{} proof intake accepted evidence", row.route));
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
            failures.push(format!("{expected_id} missing from proof intake"));
        }
    }
    failures
}
