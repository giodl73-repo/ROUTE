//! Helper `t4_terminal_columbus_source_access_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_columbus_source_access_gate_failures(
    rows: &[T4TerminalColumbusSourceAccessRow],
    intake_rows: &[T4TerminalColumbusProofIntakeRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_ids = intake_rows
        .iter()
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    if rows.is_empty() {
        failures.push("no Columbus South source access rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "Columbus source access has {} rows but expected {} intake rows",
            rows.len(),
            expected_ids.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.access_id.trim().is_empty()
            || row.intake_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.access_mode.trim().is_empty()
            || row.live_fetch_status.trim().is_empty()
            || row.required_source_metadata.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.source_access_blocker.trim().is_empty()
            || row.cache_policy_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete source access fields",
                row.access_id
            ));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in Columbus source access",
                row.queue_id
            ));
        }
        if row.terminal_district != "Columbus South" {
            failures.push(format!(
                "{} is not a Columbus South source access row",
                row.queue_id
            ));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} does not appear in the Columbus proof intake",
                row.queue_id
            ));
        }
        for required in [
            "source title",
            "source url or cached artifact",
            "capture date",
            "route",
            "terminal district",
            "route-to-terminal contact statement",
        ] {
            if !row.required_source_metadata.contains(required) {
                failures.push(format!(
                    "{} missing required source metadata {}",
                    row.queue_id, required
                ));
            }
        }
        if row.live_fetch_status != "unsupported-no-safe-terminal-fetcher" {
            failures.push(format!(
                "{} has unsupported live fetch status {}",
                row.queue_id, row.live_fetch_status
            ));
        }
        if row.acquisition_status == "source-needed" {
            if row.contact_proof_source_artifact != "source-needed"
                || row.validation_status != "review"
            {
                failures.push(format!(
                    "{} source-needed access row must keep source-needed proof artifact and review status",
                    row.queue_id
                ));
            }
            if !row
                .source_access_blocker
                .contains("no safe live terminal-contact fetch command")
            {
                failures.push(format!(
                    "{} source-needed access row lacks live-fetch blocker",
                    row.queue_id
                ));
            }
        }
        if row.acquisition_status == "source-backed"
            && row.contact_proof_source_artifact == "source-needed"
        {
            failures.push(format!(
                "{} source-backed access row lacks proof artifact",
                row.queue_id
            ));
        }
        if !matches!(
            row.acquisition_status.as_str(),
            "source-needed" | "source-backed" | "blocked"
        ) {
            failures.push(format!(
                "{} has invalid acquisition status {}",
                row.queue_id, row.acquisition_status
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review" | "held") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.queue_id, row.validation_status
            ));
        }
    }

    for expected_id in expected_ids {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} is missing from Columbus source access"
            ));
        }
    }

    failures
}
