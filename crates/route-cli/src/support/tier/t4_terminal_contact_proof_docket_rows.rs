//! Helper `t4_terminal_contact_proof_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_proof_docket_rows(
    plan_rows: &[T4TerminalContactSourcePlanRow],
    catalog_rows: &[T4TerminalContactSourceCatalogRow],
) -> Vec<T4TerminalContactProofDocketRow> {
    let catalog_by_district = catalog_rows
        .iter()
        .map(|row| (row.terminal_district.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();

    let mut rows = plan_rows
        .iter()
        .map(|row| {
            let catalog = catalog_by_district.get(row.terminal_district.as_str());
            T4TerminalContactProofDocketRow {
                task_id: format!("T4PROOF-{}", stable_id_fragment(&row.queue_id)),
                queue_id: row.queue_id.clone(),
                route: row.route.clone(),
                zone_id: row.zone_id.clone(),
                terminal_district: row.terminal_district.clone(),
                source_family: catalog
                    .map(|catalog| catalog.source_family.clone())
                    .unwrap_or_else(|| row.contact_proof_source_family.clone()),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: row.contact_proof_source_artifact.clone(),
                proof_status: "source-needed".to_string(),
                proof_blocker: row.proof_blocker.clone(),
                scenario_effect:
                    "no scenario-readiness until contact proof source and attachment are accepted"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-04.md"
                        .to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.terminal_district
            .cmp(&b.terminal_district)
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}
