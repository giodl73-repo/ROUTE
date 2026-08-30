//! Helper `tier_pavement_source_fetch_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_fetch_review_gate_failures(
    rows: &[TierPavementSourceFetchReviewRow],
    fetch_attempt_rows: &[TierPavementSourceFetchAttemptRow],
    docket_rows: &[TierPavementAcquisitionDocketRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !fetch_attempt_rows.is_empty() && rows.len() != fetch_attempt_rows.len() {
        failures.push(format!(
            "fetch review rows {} do not match fetch attempt rows {}",
            rows.len(),
            fetch_attempt_rows.len()
        ));
    }
    let docket_tasks = docket_rows
        .iter()
        .map(|row| row.task_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for row in rows {
        if row.review_id.trim().is_empty()
            || row.fetch_attempt_id.trim().is_empty()
            || row.task_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.fetch_result_status.trim().is_empty()
            || row.join_review_status.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete fetch-review row", row.task_id));
        }
        if !docket_tasks.contains(row.task_id.as_str()) {
            failures.push(format!("{} has no acquisition docket row", row.task_id));
        }
        if row.evidence_acceptance_status != "not-accepted" {
            failures.push(format!("{} accepts evidence before review", row.task_id));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!("{} reduces blockers before relief", row.task_id));
        }
        if row.cache_record_count == 0 && row.join_review_status != "fetch-repair-needed" {
            failures.push(format!(
                "{} has empty cache without fetch repair status",
                row.task_id
            ));
        }
        if row.cache_record_count > 0
            && row.postfetch_unresolved_member_count > 0
            && row.join_review_status != "cache-populated-source-gap-still-open"
        {
            failures.push(format!(
                "{} has unresolved source gap without open-gap review status",
                row.task_id
            ));
        }
        if !matches!(
            row.join_review_status.as_str(),
            "fetch-repair-needed"
                | "cache-populated-source-gap-still-open"
                | "source-gap-closed-pending-evidence-review"
        ) {
            failures.push(format!(
                "{} has invalid join review status {}",
                row.task_id, row.join_review_status
            ));
        }
    }
    failures
}
