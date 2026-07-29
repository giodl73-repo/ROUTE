//! Helper `tier_pavement_source_fetch_attempt_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_fetch_attempt_gate_failures(
    rows: &[TierPavementSourceFetchAttemptRow],
    source_access_rows: &[TierPavementSourceAccessRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !source_access_rows.is_empty() && rows.len() != source_access_rows.len() {
        failures.push(format!(
            "fetch attempt rows {} do not match source-access rows {}",
            rows.len(),
            source_access_rows.len()
        ));
    }
    for row in rows {
        if row.fetch_attempt_id.trim().is_empty()
            || row.access_policy_id.trim().is_empty()
            || row.task_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.fetch_command.trim().is_empty()
            || row.cache_target.trim().is_empty()
            || row.fetch_result_status.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete fetch-attempt row", row.task_id));
        }
        if row.evidence_acceptance_status != "not-accepted" {
            failures.push(format!("{} accepts evidence before review", row.task_id));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!("{} reduces blockers before review", row.task_id));
        }
        if row.cache_record_count == 0 && row.fetch_result_status != "fetch-failed-or-empty-cache" {
            failures.push(format!(
                "{} has empty cache without failed status",
                row.task_id
            ));
        }
    }
    failures
}

