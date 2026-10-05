//! Helper `t4_terminal_contact_proof_source_registry_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_proof_source_registry_rows(
    proof_rows: &[T4TerminalContactProofDocketRow],
    accepted_source_rows: &[T4TerminalContactAcceptedProofSourceRow],
) -> Vec<T4TerminalContactProofSourceRegistryRow> {
    let accepted_by_queue = accepted_source_rows
        .iter()
        .filter(|row| row.validation_status == "pass")
        .map(|row| (row.queue_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = proof_rows
        .iter()
        .map(|row| {
            if let Some(accepted) = accepted_by_queue.get(row.queue_id.as_str()) {
                T4TerminalContactProofSourceRegistryRow {
                    registry_id: format!("T4CONTACTREGISTRY-{}", stable_id_fragment(&row.queue_id)),
                    task_id: row.task_id.clone(),
                    queue_id: row.queue_id.clone(),
                    route: row.route.clone(),
                    terminal_district: row.terminal_district.clone(),
                    source_family: row.source_family.clone(),
                    source_artifact_mode: accepted.source_artifact_mode.clone(),
                    source_title: accepted.source_title.clone(),
                    source_url_or_cache_artifact: accepted.source_url_or_cache_artifact.clone(),
                    capture_date: accepted.capture_date.clone(),
                    contact_statement_status: "source-backed".to_string(),
                    selected_higher_tier_attachment_status: "attached".to_string(),
                    registry_status: "source-backed".to_string(),
                    proof_source_artifact: accepted.proof_source_artifact.clone(),
                    registry_blocker: "none".to_string(),
                    contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv"
                        .to_string(),
                    next_artifact:
                        "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                            .to_string(),
                    validation_status: "pass".to_string(),
                }
            } else {
                T4TerminalContactProofSourceRegistryRow {
                    registry_id: format!("T4CONTACTREGISTRY-{}", stable_id_fragment(&row.queue_id)),
                    task_id: row.task_id.clone(),
                    queue_id: row.queue_id.clone(),
                    route: row.route.clone(),
                    terminal_district: row.terminal_district.clone(),
                    source_family: row.source_family.clone(),
                    source_artifact_mode: "source-needed".to_string(),
                    source_title: "source-needed".to_string(),
                    source_url_or_cache_artifact: "source-needed".to_string(),
                    capture_date: "source-needed".to_string(),
                    contact_statement_status: "source-needed".to_string(),
                    selected_higher_tier_attachment_status: "source-needed".to_string(),
                    registry_status: "source-needed".to_string(),
                    proof_source_artifact: "source-needed".to_string(),
                    registry_blocker:
                        "manual citation or cached source artifact not registered for route-to-terminal contact proof"
                            .to_string(),
                    contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv"
                        .to_string(),
                    next_artifact:
                        "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                            .to_string(),
                    validation_status: "review".to_string(),
                }
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.terminal_district
            .cmp(&right.terminal_district)
            .then(left.route.cmp(&right.route))
    });
    rows
}
