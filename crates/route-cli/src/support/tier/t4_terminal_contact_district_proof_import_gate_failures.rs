//! Helper `t4_terminal_contact_district_proof_import_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_district_proof_import_gate_failures(
    rows: &[T4TerminalContactDistrictProofImportRow],
    registry_rows: &[T4TerminalContactProofSourceRegistryRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let Some(expected_district) = largest_registry_district(registry_rows) else {
        failures.push("no source registry rows available for district proof import".to_string());
        return failures;
    };
    let expected_ids = registry_rows
        .iter()
        .filter(|row| {
            row.terminal_district == expected_district || row.registry_status == "source-backed"
        })
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if rows.is_empty() {
        failures.push("no terminal contact district proof import rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "district proof import has {} rows but expected {} rows for {}",
            rows.len(),
            expected_ids.len(),
            expected_district
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.import_id.trim().is_empty()
            || row.registry_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.source_artifact_mode.trim().is_empty()
            || row.proof_source_artifact.trim().is_empty()
            || row.contact_statement_status.trim().is_empty()
            || row.selected_higher_tier_attachment_status.trim().is_empty()
            || row.import_status.trim().is_empty()
            || row.proof_decision.trim().is_empty()
            || row.import_blocker.trim().is_empty()
            || row.selection_rule.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete import fields", row.import_id));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in district proof import",
                row.queue_id
            ));
        }
        let selected_or_accepted = row.terminal_district == expected_district
            || registry_rows.iter().any(|registry_row| {
                registry_row.queue_id == row.queue_id
                    && registry_row.registry_status == "source-backed"
            });
        if !selected_or_accepted {
            failures.push(format!(
                "{} is outside selected district {} and has no accepted registry proof",
                row.queue_id, expected_district
            ));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} does not appear in selected district registry rows",
                row.queue_id
            ));
        }
        match row.import_status.as_str() {
            "accepted" => {
                if row.proof_decision != "source-backed"
                    || row.proof_source_artifact == "source-needed"
                    || row
                        .proof_source_artifact
                        .contains("data/intermodal_terminals.csv")
                    || row.contact_statement_status != "source-backed"
                    || row.selected_higher_tier_attachment_status != "attached"
                    || row.validation_status != "pass"
                {
                    failures.push(format!(
                        "{} accepted import lacks non-seed source-backed proof",
                        row.queue_id
                    ));
                }
            }
            "source-needed" | "blocked" | "rejected" => {
                if row.proof_decision == "source-backed" || row.validation_status == "pass" {
                    failures.push(format!(
                        "{} unresolved import cannot be source-backed/pass",
                        row.queue_id
                    ));
                }
                if row.import_blocker == "none" {
                    failures.push(format!("{} unresolved import lacks blocker", row.queue_id));
                }
            }
            other => failures.push(format!(
                "{} has invalid import status {}",
                row.queue_id, other
            )),
        }
    }
    for expected_id in expected_ids {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} is missing from terminal contact district proof import"
            ));
        }
    }
    failures
}

