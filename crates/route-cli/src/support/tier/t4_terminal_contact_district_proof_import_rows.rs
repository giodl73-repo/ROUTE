//! Helper `t4_terminal_contact_district_proof_import_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_district_proof_import_rows(
    registry_rows: &[T4TerminalContactProofSourceRegistryRow],
) -> Vec<T4TerminalContactDistrictProofImportRow> {
    let Some(selected_district) = largest_registry_district(registry_rows) else {
        return Vec::new();
    };
    let mut rows = registry_rows
        .iter()
        .filter(|row| {
            row.terminal_district == selected_district || row.registry_status == "source-backed"
        })
        .map(|row| {
            let accepted = row.registry_status == "source-backed"
                && row.proof_source_artifact != "source-needed"
                && row.contact_statement_status == "source-backed"
                && row.selected_higher_tier_attachment_status == "attached";
            T4TerminalContactDistrictProofImportRow {
                import_id: format!("T4CONTACTIMPORT-{}", stable_id_fragment(&row.queue_id)),
                registry_id: row.registry_id.clone(),
                queue_id: row.queue_id.clone(),
                route: row.route.clone(),
                terminal_district: row.terminal_district.clone(),
                source_artifact_mode: row.source_artifact_mode.clone(),
                proof_source_artifact: row.proof_source_artifact.clone(),
                contact_statement_status: row.contact_statement_status.clone(),
                selected_higher_tier_attachment_status: row
                    .selected_higher_tier_attachment_status
                    .clone(),
                import_status: if accepted {
                    "accepted".to_string()
                } else {
                    "source-needed".to_string()
                },
                proof_decision: if accepted {
                    "source-backed".to_string()
                } else {
                    "source-needed".to_string()
                },
                import_blocker: if accepted {
                    "none".to_string()
                } else {
                    row.registry_blocker.clone()
                },
                selection_rule: "largest unresolved terminal district in proof source registry"
                    .to_string(),
                next_artifact:
                    "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-04.md"
                        .to_string(),
                validation_status: if accepted {
                    "pass".to_string()
                } else {
                    "review".to_string()
                },
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
