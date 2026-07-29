//! Helper `source_fetch_policy_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn source_fetch_policy_gate_failures(rows: &[SourceFetchPolicyRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("source fetch policy emitted no rows".to_string());
        return failures;
    }
    let allowed_modes = [
        "scoped-merge",
        "full-replace-after-validation",
        "live-snapshot-preserve",
    ];
    let mut families = std::collections::BTreeSet::new();
    for row in rows {
        if !families.insert(row.fetch_family.as_str()) {
            failures.push(format!("duplicate fetch family {}", row.fetch_family));
        }
        if row.fetch_family.trim().is_empty()
            || row.commands.trim().is_empty()
            || row.cache_targets.trim().is_empty()
            || row.mutation_mode.trim().is_empty()
            || row.preservation_contract.trim().is_empty()
            || row.implementation_guard.trim().is_empty()
            || row.validation_floor.trim().is_empty()
            || row.policy_doc.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has empty policy fields", row.fetch_family));
        }
        if !allowed_modes.contains(&row.mutation_mode.as_str()) {
            failures.push(format!(
                "{} has unsupported mutation mode {}",
                row.fetch_family, row.mutation_mode
            ));
        }
        if row.validation_status != "pass" {
            failures.push(format!("{} is not policy-valid", row.fetch_family));
        }
        if !row.policy_doc.ends_with("source-fetch-cache-policy.md") {
            failures.push(format!(
                "{} does not reference policy doc",
                row.fetch_family
            ));
        }
    }
    for required in [
        "manifest-downloads",
        "hpms-national",
        "hpms-state-scope",
        "acs-county",
        "fema-corridor",
        "t1-live-event-snapshots",
    ] {
        if !families.contains(required) {
            failures.push(format!("missing source fetch family {required}"));
        }
    }
    for command in known_source_fetch_commands() {
        if !rows
            .iter()
            .any(|row| source_fetch_policy_row_covers_command(row, command))
        {
            failures.push(format!("missing source fetch command policy for {command}"));
        }
    }
    failures
}

