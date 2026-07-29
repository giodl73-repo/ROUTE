//! Helper `tier_pavement_source_access_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_access_gate_failures(
    rows: &[TierPavementSourceAccessRow],
    docket_rows: &[TierPavementAcquisitionDocketRow],
    priority: &str,
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = docket_rows
        .iter()
        .filter(|row| row.source_priority.eq_ignore_ascii_case(priority))
        .count();
    if expected > 0 && rows.len() != expected {
        failures.push(format!(
            "source-access rows {} do not match priority-{priority} docket rows {expected}",
            rows.len()
        ));
    }
    for row in rows {
        if row.access_policy_id.trim().is_empty()
            || row.task_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_access_mode.trim().is_empty()
            || row.mutation_mode.trim().is_empty()
            || row.cache_targets.trim().is_empty()
            || row.fetch_command.trim().is_empty()
            || row.preflight_gate.trim().is_empty()
            || row.postfetch_gate.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete source-access row", row.task_id));
        }
        if row.source_access_mode != "hpms-scoped-fetch" {
            failures.push(format!("{} has invalid source access mode", row.task_id));
        }
        if row.mutation_mode != "scoped-cache-merge" {
            failures.push(format!("{} has invalid mutation mode", row.task_id));
        }
        if !row.fetch_command.starts_with("route fetch-hpms --states ") {
            failures.push(format!("{} has invalid fetch command", row.task_id));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!("{} reduces blockers before evidence", row.task_id));
        }
    }
    failures
}

