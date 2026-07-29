//! Helper `tier_pavement_hpms_scope_broadening_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_hpms_scope_broadening_gate_failures(
    rows: &[TierPavementHpmsScopeBroadeningRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no HPMS scope-broadening rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.broadening_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.source_needed_routes.trim().is_empty()
            || row.current_coverage_status.trim().is_empty()
            || row.broadened_functional_systems.trim().is_empty()
            || row.broadened_fetch_command.trim().is_empty()
            || row.preflight_gate.trim().is_empty()
            || row.postfetch_gate.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete HPMS broadening row", row.state));
        }
        if row.source_needed_member_count > 0
            && row.current_hpms_records_for_source_needed_routes != 0
        {
            failures.push(format!(
                "{} already has HPMS records for source-needed routes",
                row.state
            ));
        }
        if !row
            .broadened_functional_systems
            .split(',')
            .any(|system| system == "3")
        {
            failures.push(format!(
                "{} broadened scope does not include principal arterial system 3",
                row.state
            ));
        }
        if row.source_needed_member_count > 0
            && (!row
                .broadened_fetch_command
                .starts_with("route fetch-hpms --states ")
                || !row.broadened_fetch_command.contains("--functional-systems"))
        {
            failures.push(format!("{} has invalid broadened fetch command", row.state));
        }
        if row.source_needed_member_count == 0
            && row.broadened_fetch_command != "not-required-after-broadened-fetch"
        {
            failures.push(format!(
                "{} has no source-needed members but still requires broadened fetch",
                row.state
            ));
        }
        if row.evidence_acceptance_status != "not-accepted" {
            failures.push(format!(
                "{} accepts evidence before fetch review",
                row.state
            ));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!(
                "{} reduces blockers before fetch review",
                row.state
            ));
        }
    }
    failures
}

