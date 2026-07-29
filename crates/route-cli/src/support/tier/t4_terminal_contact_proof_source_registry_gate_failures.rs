//! Helper `t4_terminal_contact_proof_source_registry_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_proof_source_registry_gate_failures(
    rows: &[T4TerminalContactProofSourceRegistryRow],
    proof_rows: &[T4TerminalContactProofDocketRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_ids = proof_rows
        .iter()
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if rows.is_empty() {
        failures.push("no terminal contact proof source registry rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "terminal contact proof source registry has {} rows but expected {} proof tasks",
            rows.len(),
            expected_ids.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.registry_id.trim().is_empty()
            || row.task_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.source_artifact_mode.trim().is_empty()
            || row.source_title.trim().is_empty()
            || row.source_url_or_cache_artifact.trim().is_empty()
            || row.capture_date.trim().is_empty()
            || row.contact_statement_status.trim().is_empty()
            || row.selected_higher_tier_attachment_status.trim().is_empty()
            || row.registry_status.trim().is_empty()
            || row.proof_source_artifact.trim().is_empty()
            || row.registry_blocker.trim().is_empty()
            || row.contract_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete source registry fields",
                row.registry_id
            ));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in terminal contact proof source registry",
                row.queue_id
            ));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} does not appear in terminal contact proof docket",
                row.queue_id
            ));
        }
        if row.source_family != "public-terminal-contact-proof" {
            failures.push(format!(
                "{} has unsupported source family {}",
                row.queue_id, row.source_family
            ));
        }
        if row
            .proof_source_artifact
            .contains("data/intermodal_terminals.csv")
        {
            failures.push(format!(
                "{} cites terminal seed data as proof",
                row.queue_id
            ));
        }
        match row.registry_status.as_str() {
            "source-backed" => {
                if !matches!(
                    row.source_artifact_mode.as_str(),
                    "manual-citation" | "cached-source-artifact"
                ) || row.source_title == "source-needed"
                    || row.source_url_or_cache_artifact == "source-needed"
                    || row.capture_date == "source-needed"
                    || row.contact_statement_status != "source-backed"
                    || row.selected_higher_tier_attachment_status != "attached"
                    || row.proof_source_artifact == "source-needed"
                    || row.validation_status != "pass"
                {
                    failures.push(format!(
                        "{} source-backed registry row lacks accepted proof artifact fields",
                        row.queue_id
                    ));
                }
            }
            "source-needed" | "blocked" | "rejected" => {
                if row.validation_status != "review" && row.validation_status != "held" {
                    failures.push(format!(
                        "{} unresolved registry row must remain review or held",
                        row.queue_id
                    ));
                }
                if row.registry_blocker.trim().is_empty() {
                    failures.push(format!(
                        "{} unresolved registry row lacks blocker",
                        row.queue_id
                    ));
                }
            }
            other => failures.push(format!(
                "{} has invalid registry status {}",
                row.queue_id, other
            )),
        }
        if row.contract_artifact != "data/t4-terminal-contact-proof-artifact-contract.csv" {
            failures.push(format!(
                "{} does not reference the proof artifact contract",
                row.queue_id
            ));
        }
    }
    for expected_id in expected_ids {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} is missing from terminal contact proof source registry"
            ));
        }
    }
    failures
}

