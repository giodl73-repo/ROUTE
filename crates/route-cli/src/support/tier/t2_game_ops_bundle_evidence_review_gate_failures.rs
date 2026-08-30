//! Helper `t2_game_ops_bundle_evidence_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_review_gate_failures(
    rows: &[T2GameOpsBundleEvidenceReviewRow],
    decision_rows: &[T2GameOpsBindingDecisionRow],
) -> Vec<String> {
    let expected = decision_rows
        .iter()
        .map(|row| row.decision_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game/ops bundle evidence review has {} rows but expected {} decisions",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.review_id.trim().is_empty()
            || row.decision_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.repair_class.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete review fields", row.review_id));
        }
        if !seen.insert(row.decision_id.clone()) {
            failures.push(format!("{} appears more than once", row.decision_id));
        }
        if !expected.contains(row.decision_id.as_str()) {
            failures.push(format!("{} is not a residual decision", row.decision_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after
            || row.blocker_count_before != row.blocker_count_after
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} does not preserve blockers", row.review_id));
        }
        if row.blocker_count_before == 0 {
            failures.push(format!("{} lacks blocker count", row.review_id));
        }
        if !matches!(
            row.validation_status.as_str(),
            "review" | "held" | "held-known"
        ) {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.review_id, row.validation_status
            ));
        }
        if row.binding_status == "bundle-bound-review"
            && (row.qualification_gate_policy.trim().is_empty()
                || row.qualification_game_use.trim().is_empty())
        {
            failures.push(format!(
                "{} bundle-bound review missing qualification semantics",
                row.review_id
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.qualification_gate_policy.trim().is_empty()
            && row.qualification_game_use.trim().is_empty()
        {
            failures.push(format!(
                "{} evidence review drops qualification contract",
                row.review_id
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from evidence review"));
        }
    }
    failures
}
