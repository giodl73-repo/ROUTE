//! Helper `optimizer_claim_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_claim_review_gate_failures(
    rows: &[OptimizerClaimReviewRow],
    backlog_rows: &[OptimizerResidualBlockerBacklogRow],
) -> Vec<String> {
    let expected = backlog_rows
        .iter()
        .filter(|row| {
            row.priority_class == "P1-claim-blocker"
                && row.next_wave == "optimizer-claim-review"
                && row.total_claim_blockers > 0
        })
        .map(|row| row.backlog_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("optimizer claim review has no P1 claim-blocker backlog rows".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "optimizer claim review has {} rows but expected {} P1 claim-blocker rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.claim_review_id.trim().is_empty()
            || row.backlog_id.trim().is_empty()
            || row.priority_class.trim().is_empty()
            || row.blocker_family.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.evidence_artifacts.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete claim-review fields",
                row.backlog_id
            ));
        }
        if !seen.insert(row.backlog_id.clone()) {
            failures.push(format!("{} appears more than once", row.backlog_id));
        }
        if !expected.contains(row.backlog_id.as_str()) {
            failures.push(format!(
                "{} is not a P1 optimizer claim-review backlog row",
                row.backlog_id
            ));
        }
        if row.priority_class != "P1-claim-blocker"
            || row.review_decision != "held-for-source-specific-claim-review"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has an invalid review state", row.backlog_id));
        }
        if row.blocker_claims_before != row.blocker_claims_after || row.claim_blocker_delta != 0 {
            failures.push(format!("{} reduced claim blockers", row.backlog_id));
        }
        if row.total_claim_blockers == 0 {
            failures.push(format!("{} has no claim blockers", row.backlog_id));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from optimizer claim review"));
        }
    }
    failures
}
